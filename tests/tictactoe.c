// Tic-tac-toe with minimax — C port of game_r.hpp / main.cpp
// clang -O0 -march=x86-64 -fno-builtin -mgeneral-regs-only -fcf-protections=none tictactoe.c -o ttt   (also try -Os)

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#define RESET "\033[0m"
#define RED   "\033[1m\033[31m"
#define GREEN "\033[1m\033[32m"
#define DEPTH 8        // how far the computer searches
#define NEG_INF (-1000) // scores are always in [-10, 10]
#define POS_INF 1000

typedef enum { PLAYER, COMPUTER, DRAW, GOING } state;

#define PLAYER_SYM   'X'
#define COMPUTER_SYM 'O'

typedef struct {
    char board[9];
} game;

typedef struct {
    game *instance;
    int iterations;
    int counter;
} computer;

typedef struct {
    game *instance;
} player;

/* ---------------- game ---------------- */

static const char* color(char c) {
    static const char *digits[] = { "1", "2", "3", "4", "5", "6", "7", "8", "9" };
    switch (c) {
    case 'X': return GREEN "X" RESET;
    case 'O': return RED "O" RESET;
    default:  return digits[c - '0' - 1];
    }
}

static void game_init(game *g) {
    for (int i = 0; i < 9; i++) g->board[i] = (char)('1' + i);
}

static void game_print(const game *g) {
    printf("    |     |    \n"
           " %s  |  %s  |  %s\n"
           "    |     |    \n"
           "----+-----+------\n"
           "    |     |    \n"
           " %s  |  %s  |  %s\n"
           "    |     |    \n"
           "----+-----+------\n"
           "    |     |    \n"
           " %s  |  %s  |  %s\n"
           "    |     |    \n\n",
           color(g->board[0]), color(g->board[1]), color(g->board[2]),
           color(g->board[3]), color(g->board[4]), color(g->board[5]),
           color(g->board[6]), color(g->board[7]), color(g->board[8]));
}

static void game_make_move(game *g, int pos, char symbol) { g->board[pos] = symbol; }
static void game_unmake_move(game *g, int pos)            { g->board[pos] = (char)(pos + '0' + 1); }
static char game_grab_cell(const game *g, int pos)        { return g->board[pos]; }

static int game_is_move_valid(const game *g, int pos) {
    return g->board[pos] != 'X' && g->board[pos] != 'O';
}

static state game_check_win(const game *g) {
    static const int win_patterns[8][3] = {
        {0, 1, 2}, {3, 4, 5}, {6, 7, 8},   // rows
        {0, 3, 6}, {1, 4, 7}, {2, 5, 8},   // cols
        {0, 4, 8}, {2, 4, 6}               // diagonals
    };
    for (int i = 0; i < 8; i++) {
        char a = g->board[win_patterns[i][0]];
        char b = g->board[win_patterns[i][1]];
        char c = g->board[win_patterns[i][2]];
        if (a == b && b == c) {
            if (a == 'X') return PLAYER;
            if (a == 'O') return COMPUTER;
        }
    }
    for (int i = 0; i < 9; i++)
        if (isdigit((unsigned char)g->board[i])) return GOING;
    return DRAW;
}

static void game_is_game_over(const game *g) {
    switch (game_check_win(g)) {
    case PLAYER:   game_print(g); printf("Player has won!\n");   exit(EXIT_SUCCESS);
    case COMPUTER: game_print(g); printf("Computer has won!\n"); exit(EXIT_SUCCESS);
    case DRAW:     game_print(g); printf("Draw!\n");             exit(EXIT_SUCCESS);
    default:       return;
    }
}

/* ---------------- player ---------------- */

static void player_make_move(player *p) {
    int num;
    for (;;) {
        num = 0;
        printf("Enter a number 1-9: ");
        int r = scanf("%d", &num);
        if (r == EOF) exit(EXIT_FAILURE);
        if (r == 1) {
            num -= 1;
            if (num >= 0 && num < 9 && game_is_move_valid(p->instance, num)) break;
        }
        printf("Wrong move please try again!\n");
        int ch;
        while ((ch = getchar()) != '\n')   // discard rest of the line
            if (ch == EOF) exit(EXIT_FAILURE);
    }
    game_make_move(p->instance, num, PLAYER_SYM);
    game_is_game_over(p->instance);
}

/* ---------------- computer ---------------- */

static int minimax(computer *c, int depth, int is_max) {
    state status = game_check_win(c->instance);

    if (status != GOING || depth == DEPTH) {
        ++c->iterations;
        if (status == COMPUTER) return 10 - depth;
        if (status == PLAYER)   return -10 + depth;
        return 0;
    }
    if (is_max) {
        int best = NEG_INF;
        for (int i = 0; i < 9; ++i) {
            if (isdigit((unsigned char)game_grab_cell(c->instance, i))) {
                game_make_move(c->instance, i, COMPUTER_SYM);
                int s = minimax(c, depth + 1, 0);
                if (s > best) best = s;
                game_unmake_move(c->instance, i);
            }
        }
        return best;
    } else {
        int best = POS_INF;
        for (int i = 0; i < 9; ++i) {
            if (isdigit((unsigned char)game_grab_cell(c->instance, i))) {
                game_make_move(c->instance, i, PLAYER_SYM);
                int s = minimax(c, depth + 1, 1);
                if (s < best) best = s;
                game_unmake_move(c->instance, i);
            }
        }
        return best;
    }
}

static int minimax_abp(computer *c, int depth, int maximizing, int alpha, int beta) {
    state status = game_check_win(c->instance);

    if (status != GOING || depth == DEPTH) {
        ++c->iterations;
        if (status == COMPUTER) return 10 - depth;
        if (status == PLAYER)   return -10 + depth;
        return 0;
    }
    if (maximizing) {
        int best = NEG_INF;
        for (int i = 0; i < 9; ++i) {
            if (isdigit((unsigned char)game_grab_cell(c->instance, i))) {
                game_make_move(c->instance, i, COMPUTER_SYM);
                int s = minimax_abp(c, depth + 1, 0, alpha, beta);
                if (s > best) best = s;
                game_unmake_move(c->instance, i);
                if (best > alpha) alpha = best;
                if (beta <= alpha) break;
            }
        }
        return best;
    } else {
        int best = POS_INF;
        for (int i = 0; i < 9; ++i) {
            if (isdigit((unsigned char)game_grab_cell(c->instance, i))) {
                game_make_move(c->instance, i, PLAYER_SYM);
                int s = minimax_abp(c, depth + 1, 1, alpha, beta);
                if (s < best) best = s;
                game_unmake_move(c->instance, i);
                if (best < beta) beta = best;
                if (beta <= alpha) break;
            }
        }
        return best;
    }
}

static void computer_make_move(computer *c) {
    int max_score = NEG_INF;
    int index = 0;
    for (int i = 0; i < 9; ++i) {
        if (isdigit((unsigned char)game_grab_cell(c->instance, i))) {
            game_make_move(c->instance, i, COMPUTER_SYM);
            int score = minimax(c, 0, 0);
            // int score = minimax_abp(c, 0, 0, NEG_INF, POS_INF);
            game_unmake_move(c->instance, i);
            if (score > max_score) {
                max_score = score;
                index = i;
            }
        }
    }
    printf("\033[1m\033[33mCPU has searched \033[1m\033[36m%d\033[0m \033[1m\033[33mmoves total\033[0m\n\n",
           c->iterations);
    c->iterations = 0;
    game_make_move(c->instance, index, COMPUTER_SYM);
    ++c->counter;
    game_is_game_over(c->instance);
}

/* ---------------- main ---------------- */

int main(void) {
    game tictactoe;
    game_init(&tictactoe);
    computer machine = { &tictactoe, 0, 0 };
    player human = { &tictactoe };
    (void)minimax_abp; // kept for parity with the original; unused like there

    char choice;
    printf("Do you want to go first[Y/N]: ");
    if (scanf(" %c", &choice) != 1) return EXIT_FAILURE;

    switch (tolower((unsigned char)choice)) {
    case 'n':
        for (;;) {
            computer_make_move(&machine);
            game_print(&tictactoe);
            player_make_move(&human);
        }
    case 'y':
        for (;;) {
            game_print(&tictactoe);
            player_make_move(&human);
            computer_make_move(&machine);
        }
    default:
        printf("Pick an option...\n");
        return EXIT_FAILURE;
    }
}
