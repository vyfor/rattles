rattle!(Dqpb, dqpb, 1, 100, ["d", "q", "p", "b"]);

rattle!(
    RollingLine,
    rolling_line,
    3,
    80,
    ["/", "-", "\\", "|", "\\", "-"]
);

rattle!(
    SimpleDots,
    simple_dots,
    3,
    400,
    [".  ", ".. ", "...", "   "]
);

rattle!(
    SimpleDotsScrolling,
    simple_dots_scrolling,
    3,
    200,
    [".  ", ".. ", "...", " ..", "  .", "   "]
);

rattle!(Arc, arc, 1, 100, ["◜", "◠", "◝", "◞", "◡", "◟"]);

rattle!(Balloon, balloon, 1, 120, [".", "o", "O", "o", "."]);

rattle!(CircleHalves, circle_halves, 1, 50, ["◐", "◓", "◑", "◒"]);

rattle!(
    CircleQuarters,
    circle_quarters,
    1,
    120,
    ["◴", "◷", "◶", "◵"]
);

rattle!(Point, point, 3, 200, ["···", "•··", "·•·", "··•", "···"]);

rattle!(SquareCorners, square_corners, 1, 180, ["◰", "◳", "◲", "◱"]);

rattle!(Toggle, toggle, 1, 250, ["⊶", "⊷"]);

rattle!(Triangle, triangle, 1, 50, ["◢", "◣", "◤", "◥"]);

rattle!(
    GrowHorizontal,
    grow_horizontal,
    1,
    120,
    ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "▊", "▋", "▌", "▍", "▎"]
);

rattle!(
    GrowVertical,
    grow_vertical,
    1,
    120,
    ["▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃"]
);

rattle!(Noise, noise, 1, 100, ["▓", "▒", "░", " ", "░", "▒"]);
