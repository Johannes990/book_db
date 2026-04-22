import re


def retrieve_render_times(lines: list[str], render_time_line_prefix: str) -> list[float]:
    render_times = []
    for line in lines:
        if line.startswith(render_time_line_prefix):
            matches = re.findall(r'-?\d*\.?\d+', line)
            render_times.append(float(matches[0]))
    
    return render_times

def load_file_content(path: str, newline: str) -> str:
    with open(path, 'r', newline=newline) as file:
        return file.readlines()
    
def get_average(vars: list[float]) -> float:
    return sum(vars) / len(vars)


if __name__ == '__main__':
    filepath = "C:\\Users\\johan\\Rust_SQLite_Book_DB\\sqlite_book_db\\libry.log"
    render_time_line_prefix = "avg frame render time:"

    lines = load_file_content(filepath, '\n')
    render_times = retrieve_render_times(lines, render_time_line_prefix)
    average = get_average(render_times)

    print(f"Average render time found: {average} ms")
    print(f"total render times found: {len(render_times)}\n")
