#include <stdint.h>

struct Game;

struct Game* _Nonnull create_game();
void destroy_game(struct Game* _Nonnull);

const char* make_score_board(struct Game*);
void free_score_board(char *);

void play_game(struct Game*, unsigned char choice);