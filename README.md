# Kifuwarabe's tic-tac-toe

〇×ゲームだぜ☆（＾～＾）  

## Test run

あなたのローカルPCでテストする方法です。  

Input:  

```shell
cargo run
```

## Run

**Dockerなんか　うまくいかね☆（＾～＾）**

あらかじめ、Docker をインストールしておいてください。  
このエグザンプルでは、動作確認は、Dockerコンテナ上の仮想Ubuntuで行うものとします。  

Dockerイメージを作成するために、以下のコマンドを打鍵してください。  

Input:  

```shell
docker build --tag tic-tac-toe:0.1 .
```

Dockerコンテナを起動するために、以下のコマンドを打鍵してください。  

Input:  

```shell
docker run --name tic-tac-toe -it --rm tic-tac-toe:0.1
```

TODO カレント・ディレクトリの移動
TODO cargo run

## How to make tic tac toe?

During development, you may need to reproduce the behavior of your computer.  
It is difficult to compare the behavior. Instead, it is useful to get the logs and compare the logs.  

* [x] Log.
  * [x] Write - Write to a file.
  * [x] Print - Write and display.

The first thing you have to create is your motive.  
It is important to start with the appearance.  

* [x] Look and Model.
  * [x] Piece - "O", "X".
  * [x] Game result - Win/Draw/Lose.
  * [x] Position - It's the board.
  * [x] Search - Computer player search info.

Let's enter commands into the computer. Create a command line parser.  

* [x] Command line parser.
  * [x] Input.
  * [x] Starts with.
  * [x] Go next to.
  * [x] Rest.

People who are looking for something 10 minutes a day are looking for something for a week in a year.  
Before creating the game itself, let's first create the replay function. Let's get it for a week.  

* [x] UXI protocol.
  * [x] Starting board.
  * [x] Player phase.
  * [x] Moves.
  * [x] To xfen.
  * [x] From xfen.

Let's make a principal command.

* [x] Main.
  * [x] position.
  * [x] pos.
  * [x] do.
  * [x] undo.
  * [x] xfen.

Before you make a computer player, let's judge the outcome. And let's test.  

* [x] Win/lose judgment.
  * [x] Win.
  * [x] Draw - Not win, not lose, can not play.
  * [-] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [x] Performance measurement.
  * [x] Node per second.

Finally, let's make a computer player.

* [x] Computer player.
  * [x] Search.
  * [-] Evaluation - None.
  * [x] Create "go" command.

# Test case

```plain
# mate +3 の局面。 5 -> [6,9]どう打っても -> [9,6]。
# x..
# o..
# o.x
position xfen x2/o2/o1x o

# mate +5 の局面？
# ...
# ...
# o.x
position xfen 3/3/o1x o

# Draw の局面。 9。
# xo.
# oxx
# xoo
position xfen xo1/oxx/xoo o
```
