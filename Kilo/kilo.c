/*** Includes {{{ ***/

#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <termios.h>
#include <unistd.h>
#include <errno.h>

// }}}

/*** Data {{{ ***/

struct termios orig_termios;

//}}}

/*** Defines {{{ ***/

#define CTRL_KEY(k) ((k) & 0x1f)

// }}}

/*** Terminal {{{ ***/

void die(const char *s) {
    write(STDOUT_FILENO, "\x1b[2J", 4);
    write(STDOUT_FILENO, "\x1b[H", 3);

    perror(s);
    exit(1);
}

void disableRawMode() {
    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios) == -1)
        die("tcsetattr");
}

void enableRawMode() {
    if (tcgetattr(STDIN_FILENO, &orig_termios) == -1) die("tcgetattr");
    atexit(disableRawMode);
    

    struct termios raw = orig_termios;
    raw.c_iflag &= ~(ICRNL | IXON | INPCK| ISTRIP | BRKINT);
    raw.c_oflag &= ~(OPOST);
    raw.c_cflag |= (CS8);
    raw.c_lflag &= ~(ECHO | ICANON | ISIG | IEXTEN);

    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;

    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1) die("tcsetattr");
}

char editorReadKey() {
    int nread;
    char c;
    while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
        if (nread == -1 && errno != EAGAIN) die("read");
    }

    return c;
}

// }}}

/*** Output {{{ ***/

void editorDrawRows() {
    int y;
    for (y = 0; y < 24; ++y) {
        write(STDOUT_FILENO, "~\r\n", 3);
    }
}

void editorRefresScreen() {
    write(STDOUT_FILENO, "\x1b[2J", 4);
    write(STDOUT_FILENO, "\x1b[H", 3);

    editorDrawRows();

    write(STDOUT_FILENO, "\x1b[H", 3);
}



//}}}

/*** Input {{{ ***/

void editorProcessKeypresses() {
    char c = editorReadKey();

    switch (c) {
        case CTRL_KEY('q'):
            write(STDOUT_FILENO, "\x1b[2J", 4);
            write(STDOUT_FILENO, "\x1b[H", 3);
            exit(0);
            break;
        /* default: */
        /*     if (iscntrl(c)) printf("%d\r\n", c); */
        /*     else printf("%d ('%c')\r\n", c, c); */
        /*     break; */
    }
}

//}}}

/*** Init {{{***/

int main() {
    enableRawMode();

    char c;
    while (1) {
        editorRefresScreen();
        editorProcessKeypresses();
    }

    return 0;
}

//}}}
