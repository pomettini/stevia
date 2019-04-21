#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define LINE_TEXT_SIZE 500
#define QUESTION_TEXT_SIZE 500

typedef enum gscript_type {
    UNDEFINED = 0,
    ERROR = 1,
    PRINT = 2,
    QUESTION = 3,
    JUMP = 4,
    END = INT_MAX
} gscript_type;

const char *gscript_text[] = {
    "P;Hello There",
    "P;I'm a VN written in the Ink format",
    "P;Do you like it?",
    "Q;Yes, I like it!;6;No, I do not like it;8",
    "P;Thank you!",
    "J;END",
    "P;Oh, I see",
    "J;END"
};

typedef struct gscript_context {
    int line_id;
    gscript_type line_type;
    char* line_text;
    int line_questions_num;
    char* line_questions_text;
} gscript_context;

gscript_context* gscript_context_init() {
    gscript_context* ctx = malloc(sizeof(gscript_context));
    ctx->line_id = 0;
    ctx->line_type = ERROR;
    ctx->line_text = malloc(LINE_TEXT_SIZE);
    ctx->line_questions_num = 0;
    ctx->line_questions_text = malloc(QUESTION_TEXT_SIZE);
    return ctx;
}

// gscript_type_t parse_line(const char* line) {
//     printf("%s\n", line);

//     if (line[0] != '+')
//         return ERROR;

//     switch (line[1])
//     {
//         case 'P':
//             return PRINT;
//             break;
        
//         case 'Q':
//             return QUESTION;
//             break;

//         case 'J':
//             return JUMP;
//             break;

//         case 'E':
//             return END;
//             break;

//         default:
//             return ERROR;
//             break;
//     }
// }

int main() {
    gscript_context* ctx = gscript_context_init();
    printf("%d", ctx->line_id);

    // while (1) {
    //     gscript_type_t parse_output = parse_line(gscript_text[ctx->index]);

    //     switch (parse_output) {
    //         case ERROR:
    //             return -1;
    //             break;

    //         case END:
    //             return 0;
    //             break;
    //     }

    //     ctx->index++;
    // }
}