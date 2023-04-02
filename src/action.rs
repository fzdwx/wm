use penrose::pure::StackSet;
use std::hash::Hash;

/**
 * 移动当前 client 到下一个 screen
 */
pub fn move_current_client_to_next_screen<C>(sc: &mut StackSet<C>)
where
    C: Clone + PartialEq + Eq + Hash,
{
    let sc_c = sc.clone();

    if let Some(c) = sc_c.current_client() {
        sc.next_screen();
        let sc_c = sc.clone();
        let tag = sc_c.current_tag();
        sc.move_client_to_tag(c, tag)
    }
}

/**
 * 移动到下一个 screen
 */
pub fn move_to_next_screen<C>(sc: &mut StackSet<C>)
where
    C: Clone + PartialEq + Eq + Hash,
{
    sc.next_screen();
}

/**
 * 切换 tag
 */
pub fn switch_tag<C>(sc: &mut StackSet<C>)
where
    C: Clone + PartialEq + Eq + Hash,
{
    sc.toggle_tag();
}

/**
 * 关闭当前 client
 */
pub fn close_current_client<C>(sc: &mut StackSet<C>)
where
    C: Clone + PartialEq + Eq + Hash,
{
    sc.kill_focused()
}
