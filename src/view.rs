use crate::board::Board;

impl Board {
    pub fn println() {
        println!(
            "\
+----+----+----+
|{6 }|{7 }|{8 }| マスを選んでください
+----+----+----+
|{3 }|{4 }|{5 }|    7 8 9
+----+----+----+    4 5 6
|{0 }|{1 }|{2 }|    1 2 3
+----+----+----+
",
            "    ", "    ", "    ", "    ", "    ", "    ", "    ", "    ", "    "
        );
    }
}
