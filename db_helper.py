import sqlite3

conn = sqlite3.connect("database.db")
cursor = conn.cursor()

cursor.execute("""
CREATE TABLE IF NOT EXISTS quotes (
    ID INTEGER PRIMARY KEY AUTOINCREMENT,
    msg TEXT NOT NULL
)
""")

messages = [
    "自然が呼んでいる、外に出て歩こう！🌿",
    "椅子は動けないけど、あなたは動ける！立ってストレッチ！🚀",
    "宠猫喊你起身活动，来吧喵 🐱",
    "Your pet cat says it’s time to move, meow! 🐱",
    "ネコが「動く時間だよ」、ニャー！🐱",
    "锻炼身体，心情也会变好哦！ 🌞",
    "Exercise your body, uplift your mood! 🌈",
    "体を動かして、気分もスッキリ！🌞",
    "别只是动动手指，起来扭扭腰吧！ 💃",
    "Don't only move your fingers, get up and wiggle your waist! 💃",
    "指だけ動かさないで、腰を振ろう！💃",
    "起身运动一下，犒劳自己一片小饼干！ 🍪",
    "Move around and reward yourself with a cookie! 🍪",
    "動いたら、クッキーを自分にご褒美！🍪",
    "再多努力五分钟，然后起来活动一下吧！ ✊",
    "Work hard for another five minutes, then get up and move! ✊",
    "あと 5 分頑張って、そして動こう！✊",
    "用挤牙膏的劲去运动，身体会感谢你！ 🏋️",
    "Squeeze in some exercise like you squeeze toothpaste, your body will thank you! 🏋️",
    "歯磨き粉を絞るように運動して、体が感謝してくれるよ！🏋️",
    "会健身的椅子才是好椅子！让你的动起来！ 🪑",
    "A good chair gets some exercise too! Let's move yours! 🪑",
    "健康な椅子は運動する椅子だ！動かそう！🪑",
    "保持健康，今天走一万步啦！ 💪",
    "Keep fit, hit 10,000 steps today! 💪",
    "健康を保つために、今日は 1 万歩歩こう！👟",
    "左脚，右脚，一步两步，动起来！ 🚶",
    "Left foot, right foot, one two step, let's move! 🚶‍♀️",
    "左右の足、一歩二歩、動こう！🚶",
    "玩个小游戏，站起来跳十次！ 🎮",
    "Play a little game, jump up ten times! 🎮",
    "ゲームしよう、10 回ジャンプ！🎮",
    "起来动一动，心情棒棒哒！ 🌟",
    "Get up and move, feel super awesome! 🌟",
    "立ち上がって動くと、気分が最高！✨",
    "给自己来个 “动感时刻” 吧！ 🕺",
    "自分に「動の時間」を贈ろう！🕺",
    "收拾收拾，活动一下筋骨！ 🚶",
    "Tidy up and stretch your muscles! 🚶‍♂️",
    "身の回りを整理して、筋肉を伸ばそう！🧹",
    "休息片刻，能量满满！ ✨",
    "Take a break, feel energized! ✨",
    "休憩して、エネルギーを補充しよう！🔋",
    "运动一下，神清气爽！ 🌿",
    "A quick workout, clear mind! 🌿",
    "少し運動して、頭をリフレッシュ！🍃",
    "每天一小步，健康一大步！ 🤸",
    "A little step each day, a big leap to health! 🤸",
    "毎日少し動いて、大きな健康を得よう！🏃‍♂️",
    "起身喝口水，再散步五分钟！ 🚶",
    "Stand up, have a sip of water, and take a five-minute walk! 🚶‍♀️",
    "立ち上がって水を飲んで、5 分歩こう！🚶‍♂️",
    "时间到！该起来动动啦！ ⏰",
    "Time's up! Get up and move around! ⏰",
    "時間だよ！立ち上がって動こう！⏳",
    "只需几步，就能解锁健康！ 🚶",
    "Just a few steps to unlock health! 🚶‍♀️",
    "健康へのカギは、数歩歩くだけ！🔑",
    "活动一下，用笑容迎接每一天！ 😊",
    "Move around, greet each day with a smile! 😊",
    "動いて、毎日を笑顔で迎えよう！😊",
    "别让自己 “长在” 椅子上，起来动动吧！ 🌟",
    "椅子に「根を張らない」ように、立ち上がって動こう！🌿",
    "让你的小腿跑一跑，充电完毕！⚡",
    "Let your legs run a bit, recharge complete! ⚡",
    "少し足を動かして、充電完了！⚡",
    "记得活动哦！短暂休息，长久健康！ 🚶",
    "Don't forget to move! Short breaks, long health! 🚶‍♂️",
    "動くのを忘れないで！短い休憩、長い健康！🚶‍♀️",
    "起身摇摆，工作更自在！ 💃",
    "Stand up and sway, work feels like play! 💃",
    "立ち上がって揺れよう、仕事が楽しくなるよ！💃 ",
    "久坐不好，起来活动活动吧！ 🚶",
    "记得起身活动活动，松松背拉拉筋。😊",
]


for idx, msg in enumerate(messages, start=1):
    cursor.execute(
        """
    INSERT INTO quotes (ID, msg) VALUES (?, ?)
    """,
        (idx, msg),
    )

conn.commit()
conn.close()
