from dataclasses import dataclass, field


text_col = "text"
text_alt_col = "text_alt"
text_highlight_col = "text_highlight"
back_col = "background"
back_alt_col = "background_alt"
back_highlight_col = "background_highlight"
warning_col = "warning"
error_col = "error"
border_col = "border"
accent_col = "accent"

all_colors = {
    text_col,
    text_alt_col,
    text_highlight_col,
    back_col,
    back_alt_col,
    back_highlight_col,
    warning_col,
    error_col,
    border_col,
    accent_col
}

MOD_ITALIC = "ITALIC"
MOD_UNDERLINED = "UNDERLINED"
MOD_BOLD = "BOLD"


@dataclass(frozen=True)
class Style:
    fg: str | None
    bg: str | None = None
    modifiers: frozenset[str] | None = None
    id: int = field(default=0, compare=False)

    def __init__(self, fg, bg=None, modifiers=None):
        object.__setattr__(self, "fg", fg)
        object.__setattr__(self, "bg", bg)
        object.__setattr__(self, "modifiers", modifiers)
        object.__setattr__(self, "id", Style.id)
        Style.id += 1

    def __str__(self):
        return f"Style <ID: {self.id:4d}> - foreground: {self.fg}, background: {self.bg}, modifiers: {self.modifiers}"
    
    def __repr__(self):
        return f"Style(fg={self.fg}, bg={self.bg}, modifiers={self.modifiers})"
    
    def covers(self, other: "Style") -> bool:
        if self.fg != other.fg:
            return False
        
        if self.bg != other.bg:
            return False
        
        if self.modifiers != other.modifiers:
            return False
        
        return True


def normalize(s: Style):
    return (
        s.fg,
        s.bg,
        tuple(sorted(s.modifiers)) if s.modifiers else None
    )


def minimal_cover(styles: list[Style]) -> list[Style]:
    uncovered = set(styles)
    result = []

    while uncovered:
        best = None
        best_coverage = set()

        for s in styles:
            coverage = {u for u in uncovered if s.covers(u)}
            if len(coverage) > len(best_coverage):
                best = s
                best_coverage = coverage
        
        result.append(best)
        uncovered -= best_coverage
    
    return result


def visualize_clusters(selected: list[Style], all_styles: list[Style]):
    for base in selected:
        print(base)
        
        covered = [s for s in all_styles if base.covers(s)]
        
        for s in covered:
            print(f"  ├── {s}")
        
        print(f"  └── total: {len(covered)}\n")


if __name__ == '__main__':
    #render splash screen
    styles = []
    splash_main_page_style = Style(text_col, back_col)
    styles.append(splash_main_page_style)

    #info block
    info_style = Style(border_col, back_alt_col)
    styles.append(info_style)

    #file explorer screen
    fex_scrollbar_style = Style(border_col)
    fex_page_style = Style(text_col, back_col)
    fex_0_line_style = Style(text_col, back_alt_col)
    fex_1_line_style = Style(text_col, back_col)
    fex_highlight_style = Style(text_highlight_col, back_highlight_col)
    fex_border_block_style = Style(border_col, back_col)
    styles.append(fex_scrollbar_style)
    styles.append(fex_page_style)
    styles.append(fex_0_line_style)
    styles.append(fex_1_line_style)
    styles.append(fex_highlight_style)
    styles.append(fex_border_block_style)

    #database schema screen
    db_schema_page_style = Style(text_col, back_col)
    styles.append(db_schema_page_style)

    #render table list
    render_table_scrollbar_style = Style(border_col, back_col)
    render_table_row_style = Style(text_col, back_col)
    render_table_border_block_style = Style(border_col, back_col)
    render_table_highlight_style = Style(text_highlight_col, back_highlight_col)
    styles.append(render_table_scrollbar_style)
    styles.append(render_table_row_style)
    styles.append(render_table_border_block_style)
    styles.append(render_table_highlight_style)

    #render column list
    render_col_scrollbar_style = Style(border_col)
    render_col_header_style = Style(text_col)
    render_col_highlight_style = Style(text_highlight_col, back_highlight_col)
    render_col_border_block_style = Style(border_col, back_col)
    styles.append(render_col_scrollbar_style)
    styles.append(render_col_header_style)
    styles.append(render_col_highlight_style)
    styles.append(render_col_border_block_style)

    #new database screen
    new_db_page_style = Style(text_alt_col, back_col)
    new_db_insert_text_area_on_style = Style(text_highlight_col, back_highlight_col)
    styles.append(new_db_page_style)
    styles.append(new_db_insert_text_area_on_style)

    #render DB table screen
    render_db_table_page_style = Style(text_alt_col, back_col)
    render_db_col_name_style = Style(text_col, None, frozenset([MOD_ITALIC, MOD_UNDERLINED]))
    render_db_metadata_style = Style(text_alt_col, None, frozenset([MOD_ITALIC]))
    render_db_scrollbar_style = Style(border_col)
    render_db_highlight_style = Style(text_highlight_col, back_highlight_col)
    render_db_border_block_style = Style(border_col, back_col)
    styles.append(render_db_table_page_style)
    styles.append(render_db_col_name_style)
    styles.append(render_db_metadata_style)
    styles.append(render_db_scrollbar_style)
    styles.append(render_db_highlight_style)
    styles.append(render_db_border_block_style)

    #render options screen
    render_options_page_style = Style(text_col, back_col)
    render_options_selected_style = Style(error_col)
    render_options_border_block_style = Style(border_col, back_col)
    render_options_highlight_style = Style(text_highlight_col, back_highlight_col)
    render_options_selected_row_style = Style(text_col, None, frozenset([MOD_BOLD]))
    render_options_not_selected_row_style = Style(text_col)
    styles.append(render_options_page_style)
    styles.append(render_options_selected_style)
    styles.append(render_options_border_block_style)
    styles.append(render_options_highlight_style)
    styles.append(render_options_selected_row_style)
    styles.append(render_options_not_selected_row_style)

    #render quit popuop
    render_quit_popup_style = Style(text_col, warning_col)
    styles.append(render_quit_popup_style)

    #render no db loaded popup
    render_nodb_popup_style = Style(text_col, warning_col)
    styles.append(render_nodb_popup_style)

    #render insert row popup
    render_insert_row_popup_style = Style(text_col, back_alt_col)
    render_insert_row_metadata_style = Style(text_alt_col, None, frozenset([MOD_ITALIC]))
    render_insert_row_text_on_style = Style(text_highlight_col, back_highlight_col)
    render_insert_row_text_off_style = Style(text_col, back_alt_col)
    styles.append(render_insert_row_popup_style)
    styles.append(render_insert_row_metadata_style)
    styles.append(render_insert_row_text_on_style)
    styles.append(render_insert_row_text_off_style)

    #render insert sql popup
    render_insert_sql_popup_style = Style(text_col, back_alt_col)
    render_insert_sql_text_in_style = Style(text_highlight_col, back_highlight_col)
    render_insert_sql_text_off_style = Style(text_col, back_alt_col)
    styles.append(render_insert_sql_popup_style)
    styles.append(render_insert_sql_text_in_style)
    styles.append(render_insert_sql_text_off_style)

    #render insert table popup
    render_insert_table_popup_style = Style(text_col, back_alt_col)
    render_insert_table_border_block_stype = Style(border_col, back_col)
    render_insert_table_highlight_style = Style(text_highlight_col, back_highlight_col)
    render_insert_table_scrollbar_style = Style(border_col)
    styles.append(render_insert_table_popup_style)
    styles.append(render_insert_table_border_block_stype)
    styles.append(render_insert_table_highlight_style)
    styles.append(render_insert_table_scrollbar_style)

    #render drop table popup
    render_drop_table_popup_style = Style(text_col, back_alt_col)
    render_drop_table_text_area_style = Style(text_highlight_col, back_highlight_col)
    styles.append(render_drop_table_popup_style)
    styles.append(render_drop_table_text_area_style)

    #render delete row popup
    render_delete_row_popup_style = Style(text_col, back_alt_col)
    render_delete_row_text_area_on_style = Style(text_col, back_highlight_col)
    render_delete_row_text_area_off_style = Style(text_col, back_alt_col)
    styles.append(render_delete_row_popup_style)
    styles.append(render_delete_row_text_area_on_style)
    styles.append(render_delete_row_text_area_off_style)

    #render error popup
    render_error_popup_style = Style(text_col, error_col)
    render_error_block_style = Style(border_col)
    styles.append(render_error_popup_style)
    styles.append(render_error_block_style)

    print("Calculating minimal covering set...")
    minimal_set = minimal_cover(styles=styles)

    print(minimal_set)
    visualize_clusters(minimal_set, styles)