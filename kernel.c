#include <stddef.h>
#include <stdint.h>

#define VGA_WIDTH 80
#define VGA_HEIGHT 25
#define VGA_MEMORY ((uint16_t*) 0xB8000)

enum vga_color {
    VGA_COLOR_BLACK = 0,
    VGA_COLOR_BLUE = 1,
    VGA_COLOR_GREEN = 2,
    VGA_COLOR_CYAN = 3,
    VGA_COLOR_RED = 4,
    VGA_COLOR_MAGENTA = 5,
    VGA_COLOR_BROWN = 6,
    VGA_COLOR_LIGHT_GREY = 7,
    VGA_COLOR_DARK_GREY = 8,
    VGA_COLOR_LIGHT_BLUE = 9,
    VGA_COLOR_LIGHT_GREEN = 10,
    VGA_COLOR_LIGHT_CYAN = 11,
    VGA_COLOR_LIGHT_RED = 12,
    VGA_COLOR_LIGHT_MAGENTA = 13,
    VGA_COLOR_YELLOW = 14,
    VGA_COLOR_WHITE = 15,
};

static inline uint8_t make_color(enum vga_color fg, enum vga_color bg) {
    return fg | (bg << 4);
}

static inline uint16_t make_vga_entry(char c, uint8_t color) {
    return (uint16_t) c | ((uint16_t) color << 8);
}

void clear_screen(uint8_t color) {
    for (size_t y = 0; y < VGA_HEIGHT; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            const size_t index = y * VGA_WIDTH + x;
            VGA_MEMORY[index] = make_vga_entry(' ', color);
        }
    }
}

void print_string_at(const char* str, size_t x, size_t y, uint8_t color) {
    size_t index = y * VGA_WIDTH + x;
    for (size_t i = 0; str[i] != '\0'; i++) {
        VGA_MEMORY[index++] = make_vga_entry(str[i], color);
    }
}

void kernel_main(void) {
    uint8_t bg_color     = make_color(VGA_COLOR_WHITE, VGA_COLOR_BLUE);
    uint8_t header_color = make_color(VGA_COLOR_YELLOW, VGA_COLOR_BLUE);
    uint8_t text_color   = make_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLUE);
    uint8_t alert_color  = make_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLUE);

    clear_screen(bg_color);

    print_string_at("+------------------------------------------------------------------------------+", 0, 1, header_color);
    print_string_at("|                               CARLINHO OS v1.0                               |", 0, 2, header_color);
    print_string_at("+------------------------------------------------------------------------------+", 0, 3, header_color);

    print_string_at("SISTEMA DE PROCESSAMENTO DE DADOS EMPRESARIAIS", 17, 6, bg_color);

    print_string_at("[1] Executar Rotina Comercial (FATURAMENTO)", 15, 10, text_color);
    print_string_at("[2] Acessar Banco SABIA (PROCESSAMENTO COBOL)", 15, 12, text_color);
    print_string_at("[3] Emitir Relatorio de Contas a Pagar / Receber", 15, 14, text_color);
    print_string_at("[4] Modulo de Teleprocessamento e Lote", 15, 16, text_color);
    print_string_at("[5] Encerrar Sessao / Standby", 15, 18, text_color);

    print_string_at("STATUS: AGUARDANDO COMANDO DO OPERADOR...", 2, 22, alert_color);
    print_string_at("MODO: 32-BIT BARE-METAL MULTIBOOT", 2, 23, make_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLUE));
}
