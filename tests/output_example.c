#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>

#define LINE_TEXT_SIZE 200
#define QUESTIONS_TEXT_SIZE 200
#define MAX_QUESTION_SIZE 2
#define SCRIPT_LEN 8

#define ERROR 0
#define OK 1

typedef enum gscript_type
{
    // TYPE_UNDEFINED = 0,
    // TYPE_ERROR = 1,
    TYPE_PRINT = 2,
    TYPE_QUESTION = 3,
    // TYPE_JUMP = 4,
    // TYPE_END = INT_MAX
} gscript_type;

typedef struct gscript_context
{
    char **script;
    int line_id;
    gscript_type line_type;
    char *line_text;
    int line_questions_num;
    char **line_questions_text;
    int *line_questions_jump;
} gscript_context;

char *alloc_string(int size, char *string);
char **gscript_text_init();
gscript_context *gscript_context_init(char **script);
int gscript_parse_question(gscript_context *ctx);
void gscript_parse_print(gscript_context *ctx);
void gscript_parse_jump(gscript_context *ctx);
gscript_type gscript_parse_line(char *line);
int gscript_process_line(gscript_context *ctx);
int gscript_print_line(gscript_context *ctx);
void debug_gscript_print_status(gscript_context *ctx);

char *alloc_string(int size, char *string)
{
    char *str = malloc(size);
    str = string;
    return str;
}

char **gscript_text_init()
{
    char **gscript_text;
    gscript_text = malloc(SCRIPT_LEN);
    gscript_text[0] = alloc_string(14, "P;Hello There");
    gscript_text[1] = alloc_string(37, "P;I'm a VN written in the Ink format");
    gscript_text[2] = alloc_string(18, "P;Do you like it?");
    gscript_text[3] = alloc_string(43, "Q;Yes, I like it!;4;No, I do not like it;6");
    gscript_text[4] = alloc_string(13, "P;Thank you!");
    gscript_text[5] = alloc_string(6, "J;END");
    gscript_text[6] = alloc_string(12, "P;Oh, I see");
    gscript_text[7] = alloc_string(6, "J;END");
    return gscript_text;
}

gscript_context *gscript_context_init(char **script)
{
    gscript_context *ctx = malloc(sizeof(gscript_context));
    ctx->script = script;
    ctx->line_id = 0;
    ctx->line_type = ERROR;
    ctx->line_text = malloc(LINE_TEXT_SIZE);
    ctx->line_questions_num = 0;
    ctx->line_questions_text = malloc(MAX_QUESTION_SIZE);
    ctx->line_questions_jump = malloc(MAX_QUESTION_SIZE);

    for (int i = 0; i < MAX_QUESTION_SIZE; i++)
        ctx->line_questions_text[i] = malloc(QUESTIONS_TEXT_SIZE);

    return ctx;
}

int gscript_parse_question(gscript_context *ctx)
{
    char *token;
    char sample[] = "Q;Yes, I like it!;4;No, I do not like it;6";

    ctx->line_questions_num = 0;
    int count = 0;

    // Skipping the first two characters
    // Because they're useless
    for (token = strtok(sample + 2, ";"); token; token = strtok(NULL, ";"))
    {
        // printf("%d: %s\n", count, token);

        // Counts to multiple of two
        // The first is going to be the text
        // The second is going to be the index
        if ((count + 1) % 2) 
        {
            // If is't the text
            // printf("%s\n", token);
            ctx->line_questions_text[ctx->line_questions_num] = token;
        }
        else
        {
            // If it's the jump index
            // printf("%d\n", atoi(token));
            ctx->line_questions_jump[ctx->line_questions_num] = atoi(token);
            ctx->line_questions_num++;
        }

        count++;
    }

    // for (int i = 0; i < ctx->line_questions_num; i++)
    // {
    //     printf("%s jumps to %d\n", ctx->line_questions_text[i], ctx->line_questions_jump[i]);
    // }

    return 1;
}

void gscript_parse_print(gscript_context *ctx)
{
    // Skipping the first two characters
    // Because they're useless
    ctx->line_text = ctx->script[ctx->line_id] + 2;
}

void gscript_parse_jump(gscript_context *ctx)
{
    // ctx->line_id =
}

gscript_type gscript_parse_line(char *line)
{
    switch (line[0])
    {
    case 'P':
        return TYPE_PRINT;
        break;

    case 'Q':
        return TYPE_QUESTION;
        break;

    // case 'J':
    //     return TYPE_JUMP;
    //     break;

    // case 'E':
    //     return TYPE_END;
    //     break;

    // default:
    //     return TYPE_ERROR;
    //     break;
    }

    return ERROR;
}

int gscript_process_line(gscript_context *ctx)
{
    switch (ctx->line_type)
    {
    case TYPE_PRINT:
        gscript_parse_print(ctx);
        return OK;
        break;

    case TYPE_QUESTION:
        gscript_parse_question(ctx);
        return OK;
        break;
    }

    return ERROR;
}

int gscript_print_line(gscript_context *ctx)
{
    switch (ctx->line_type)
    {
    case TYPE_PRINT:
        printf("- %s\n", ctx->line_text);
        return OK;
        break;

    case TYPE_QUESTION:
        for (int i = 0; i < ctx->line_questions_num; i++)
        {
            printf("- * %s -> %d\n", ctx->line_questions_text[i], ctx->line_questions_jump[i]);
        }
        return OK;
        break;
    }

    return ERROR;
}

void debug_gscript_print_status(gscript_context *ctx)
{
    printf("%d\n", ctx->line_id);
    printf("%d\n", ctx->line_type);
    printf("%s\n", ctx->line_text);
    printf("%d\n", ctx->line_questions_num);
    printf("\n");
}

int main()
{
    char **gscript_text = gscript_text_init();
    gscript_context *ctx = gscript_context_init(gscript_text);

    int result = 0;

    while (1)
    {
        // First, parse line
        ctx->line_type = gscript_parse_line(ctx->script[ctx->line_id]);

        // Second, process line
        result = gscript_process_line(ctx);

        // Debug: print line
        result = gscript_print_line(ctx);

        // debug_gscript_print_status(ctx);

        ctx->line_id += 1;

        if (ctx->line_id >= SCRIPT_LEN)
            break;
    }

    return 0;
}