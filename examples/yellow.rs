
use pdcurses::*;

macro_rules! check_err {
    (
        $($e:expr;)+
    ) => {
        $(
            if { $e } == ERR {
                panic!(concat!("`", stringify!($e), "` failed!"));
            }
        )+
    };
}

fn main() {
    let scr = initscr();
    assert_eq!(scr, stdscr());

    let mut scr_y = 0;
    let mut scr_x = 0;
    getmaxyx(scr, &mut scr_y, &mut scr_x);
    println!("SCREEN X = {}\nSCREEN Y = {}", scr_x, scr_y);

    let mut pdc_version = unsafe { std::mem::zeroed() };
    PDC_get_version(&mut pdc_version);
    println!("UTF8 = {}", pdc_version.flags & PDC_VFLAG_UTF8 != 0);
    println!("WIDE = {}", pdc_version.flags & PDC_VFLAG_WIDE != 0);

    println!("has_colors = {}", has_colors());
    println!("can_change_color = {}", can_change_color());

    check_err! {
        start_color();
        init_pair(1, COLOR_YELLOW, COLOR_BLACK);
        attr_set(A_BOLD, 1);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    let text = "This somewhat long bright (bold) yellow text will stay on screen for approximately 3 seconds.";
    let text_n = text.len() as i32;
    check_err! {
        mvaddstr(scr_y/2, scr_x/2 - text_n/2, text);
        refresh();
    }

    std::thread::sleep(std::time::Duration::from_secs(3));

    check_err! {
        endwin();
    }
}
