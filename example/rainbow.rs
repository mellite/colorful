extern crate colorful;

use colorful::Colorful;

fn main() {
    let text = format!("{:^50}\n{}\r\n{}", "岳飞 小重山", "昨夜寒蛩不住鸣 惊回千里梦 已三更 起身独自绕阶行 人悄悄 帘外月胧明",
                       "白首为功名 旧山松竹老 阻归程 欲将心事付瑶琴 知音少 弦断有谁听");
    text.rainbow();
}