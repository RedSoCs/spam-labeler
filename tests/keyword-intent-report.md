# Keyword Intent Extraction Report

Generated against: `data/spam.txt`

## Techniques

1. **Prefix Keyword Extraction** — Extract first word before space as primary intent keyword
2. **Tiktoken Sub-tokenization** — Split mixed Thai/English prefixes (e.g., `888NEO` → `888`, `neo`)
3. **Intent Dictionary Scanning** — RAKE-style keyword extraction from body text

---

## Intent Distribution Summary

- **Total samples analyzed**: 365
- **Total keywords extracted**: 2569

| Intent | Count | Percentage |
|--------|-------|------------|
| Navigational | 132 | 36.2% |
| Commercial | 93 | 25.5% |
| Informational | 88 | 24.1% |
| Transactional | 35 | 9.6% |
| Unknown | 17 | 4.7% |

---

## Per-Sample Results

### Sample 1

- **Input**: `1000แท้ role= สนุกกับโลกคาสิโนออนไลน์อย่างที่ไม่เคยรู้สึกมาก่อน! เพียงแค่ลงทะเบียน คุณจะกลายเป็นสมาช`
- **Prefix keyword**: `Some("1000")`
- **Sub-tokens**: `["100"]`
- **Primary intent**: `Commercial` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| 1000 | 1.00 |
| 100 | 0.80 |
| พิเศษ | 0.40 |
| รางวัล | 0.20 |
| vip | 0.20 |
| สมาชิก | 0.20 |
| ลงทะเบียน | 0.20 |
| โปรโมชั่น | 0.20 |
| ของขวัญ | 0.20 |

### Sample 2

- **Input**: `123 SERIES รวม SpadeGaming ที่เล่นลื่นและมีลูกเล่นพอดี ธีมหลากหลายทำให้ไม่เบื่อ เหมาะกับคนที่ชอบสลับ`
- **Prefix keyword**: `Some("123")`
- **Sub-tokens**: `["123"]`
- **Primary intent**: `Informational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| 123 | 1.00 |
| เล่น | 0.60 |
| spade | 0.20 |

### Sample 3

- **Input**: `14.3 x 4.5 x 17 inches; 7.41 ounces. Type of item, Video Game. Item model number, 4655518633545. Is `
- **Prefix keyword**: `Some("14")`
- **Sub-tokens**: `["14"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| 14 | 1.00 |
| ufa | 0.20 |

### Sample 4

- **Input**: `248 สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้การเล`
- **Prefix keyword**: `Some("248")`
- **Sub-tokens**: `["248"]`
- **Primary intent**: `Informational` (confidence: 40%)

| Keyword | Score |
|---------|-------|
| 248 | 1.00 |
| เล่น | 0.60 |
| สล็อต | 0.20 |
| แนะนำ | 0.20 |

### Sample 5

- **Input**: `33หวยหุ้น 5 เว็บพนันสล็อตบาคาร่า แอปสล็อตเว็บใหญ่ บาคาร่า sexy ลุ้นไพ่ อีเวนต์สล็อตบาคาร่า โอกาสรวยจ`
- **Prefix keyword**: `Some("33")`
- **Sub-tokens**: `["33"]`
- **Primary intent**: `Navigational` (confidence: 46%)

| Keyword | Score |
|---------|-------|
| 33 | 1.00 |
| บาคาร่า | 0.60 |
| สล็อต | 0.60 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |

### Sample 6

- **Input**: `4×4 สล็อต ไม่เด้ง เว็บสล็อตออนไลน์ครบวงจรที่รวมเกมสล็อตยอดนิยมไว้มากมาย รองรับการใช้งานผ่าน slot wal`
- **Prefix keyword**: `None`
- **Sub-tokens**: `[]`
- **Primary intent**: `Transactional` (confidence: 67%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| wallet | 0.33 |
| เล่น | 0.33 |
| ขั้นต่ำ | 0.33 |
| slot | 0.33 |
| ถอน | 0.33 |
| ฝาก | 0.33 |

### Sample 7

- **Input**: `4KAUTO Roullete ช่วยเพิ่มความตื่นเต้นด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่ถนัด ตั้งเป้าเป็นรอบ และเล`
- **Prefix keyword**: `Some("4kauto")`
- **Sub-tokens**: `["auto"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| 4kauto | 1.00 |
| auto | 0.80 |
| เล่น | 0.40 |

### Sample 8

- **Input**: `4X4SLOT สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้ก`
- **Prefix keyword**: `Some("4x4slot")`
- **Sub-tokens**: `["slot"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| 4x4slot | 1.00 |
| slot | 0.80 |
| เล่น | 0.60 |
| แนะนำ | 0.20 |
| สล็อต | 0.20 |

### Sample 9

- **Input**: `69GOAL สัมผัส Live Casino กับดีลเลอร์จริง ภาพสดเสียงชัด ระบบลื่นบนมือถือ เหมาะกับคนที่ชอบความเรียลแล`
- **Prefix keyword**: `Some("69goal")`
- **Sub-tokens**: `["69", "goal"]`
- **Primary intent**: `Navigational` (confidence: 61%)

| Keyword | Score |
|---------|-------|
| 69goal | 1.00 |
| 69 | 0.80 |
| goal | 0.80 |
| เล่น | 0.60 |
| casino | 0.40 |

### Sample 10

- **Input**: `6 days ago — ผลบอลสดมีเสียงเตือน888 ที่รู้จักกัน แพลตฟอร์ม PG Slot ชั้นนำในประเทศไทย พร้อมสิทธิพิเศษ`
- **Prefix keyword**: `None`
- **Sub-tokens**: `[]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| pg slot | 1.00 |
| สมาชิก | 1.00 |
| พิเศษ | 1.00 |
| slot | 1.00 |

### Sample 11

- **Input**: `789GOLD LOGIN รวม สล็อตออนไลน์ ที่คัดเกมเล่นง่าย โบนัสเข้าไว เมนูไม่ซับซ้อน รองรับมือถือทุกค่าย เหมา`
- **Prefix keyword**: `Some("789gold")`
- **Sub-tokens**: `["789", "gold"]`
- **Primary intent**: `Informational` (confidence: 12%)

| Keyword | Score |
|---------|-------|
| 789gold | 1.00 |
| 789 | 0.80 |
| gold | 0.80 |
| เล่น | 0.40 |
| โบนัส | 0.20 |
| สล็อต | 0.20 |

### Sample 12

- **Input**: `7M2IN1 Roullete ช่วยเพิ่มความตื่นเต้นด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่ถนัด ตั้งเป้าเป็นรอบ และเล`
- **Prefix keyword**: `Some("7m2in1")`
- **Sub-tokens**: `["in"]`
- **Primary intent**: `Navigational` (confidence: 36%)

| Keyword | Score |
|---------|-------|
| 7m2in1 | 1.00 |
| in | 0.80 |
| เล่น | 0.40 |

### Sample 13

- **Input**: `7m-888 ไม่ว่าคุณจะชอบรูปแบบการหมุนวงล้อ หรือคนรักกีฬา ที่นี่มีให้ครบ พร้อมสิทธิ์ทดลอง และของขวัญหลัง`
- **Prefix keyword**: `Some("7m")`
- **Sub-tokens**: `["7m"]`
- **Primary intent**: `Commercial` (confidence: 11%)

| Keyword | Score |
|---------|-------|
| 7m | 1.00 |
| ของขวัญ | 0.20 |
| 888 | 0.20 |
| ลงทะเบียน | 0.20 |
| ทดลอง | 0.20 |

### Sample 14

- **Input**: `7MTH เลือก Slot ที่ออกแบบหน้าเว็บให้ใช้งานสบาย กดเข้าเกมไว ลดการรอคอย เล่นต่อเนื่องได้ดี เหมาะกับคนท`
- **Prefix keyword**: `Some("7mth")`
- **Sub-tokens**: `["th"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| 7mth | 1.00 |
| th | 0.80 |
| เล่น | 0.20 |
| slot | 0.20 |

### Sample 15

- **Input**: `888 livescoreยิ่2026 ระบบใหม่ล่าสุด ทดลองฟรีเลย ระบบอัตโนมัติล่าสุด ฝากถอน 30 วินาที ไม่มีค่าธรรมเนี`
- **Prefix keyword**: `Some("888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Navigational` (confidence: 60%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| livescore | 0.20 |
| ถอน | 0.20 |
| ฝาก | 0.20 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |

### Sample 16

- **Input**: `888NEO LOGIN รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่ชอบ`
- **Prefix keyword**: `Some("888neo")`
- **Sub-tokens**: `["888", "neo"]`
- **Primary intent**: `Navigational` (confidence: 71%)

| Keyword | Score |
|---------|-------|
| 888neo | 1.00 |
| 888 | 0.80 |
| neo | 0.80 |
| เล่น | 0.40 |
| jili | 0.20 |
| slot | 0.20 |

### Sample 17

- **Input**: `888บอล เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะเพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้งสล็อต รูเล็ต`
- **Prefix keyword**: `Some("888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Navigational` (confidence: 88%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| รูเล็ต | 0.20 |
| ลงทะเบียน | 0.20 |
| สล็อต | 0.20 |

### Sample 18

- **Input**: `*888 โปรโมชั่นฟรี! ไม่ต้องฝาก เครดิตเดบิต ไม่จำกัด ทดลองใช้ ฟรีโบนัส สมาชิก ... *888 จากของขวัญแต้มส`
- **Prefix keyword**: `Some("888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Commercial` (confidence: 64%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| ฟรี | 0.40 |
| โปรโมชั่น | 0.40 |
| ฝาก | 0.20 |
| สมาชิก | 0.20 |
| ทดลอง | 0.20 |
| ไม่ต้องฝาก | 0.20 |
| ของขวัญ | 0.20 |

### Sample 19

- **Input**: `888ผลบอล ❤️ ใช้กลยุทธ์ลับเพื่อชนะรางวัลใหญ่ในรูเล็ต ทำให้แน่ใจว่าคุณจะได้รับประโยชน์สูงสุดจากทุกการห`
- **Prefix keyword**: `Some("888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Navigational` (confidence: 86%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| รางวัล | 0.20 |
| รูเล็ต | 0.20 |

### Sample 20

- **Input**: `888ผลบอลสดพร้อมราคา เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะเพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้`
- **Prefix keyword**: `Some("888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Navigational` (confidence: 70%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| รูเล็ต | 0.20 |
| ฝากเงิน | 0.20 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝาก | 0.20 |

### Sample 21

- **Input**: `888 หวย เพียงแค่เข้าสู่ระบบ คุณสามารถทดลองเล่นเกมทุกรุ่นล่าสุดได้เลยโดยไม่จำเป็นต้องฝากเงิน! แล้วยัง`
- **Prefix keyword**: `Some("888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Transactional` (confidence: 42%)

| Keyword | Score |
|---------|-------|
| 888 | 1.00 |
| เล่น | 0.40 |
| เข้าสู่ระบบ | 0.20 |
| พิเศษ | 0.20 |
| ทดลอง | 0.20 |
| ฝากเงิน | 0.20 |
| ฝาก | 0.20 |

### Sample 22

- **Input**: `88PLUS เลือก Joker ที่เล่นแล้วคุ้นมือ กติกาไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่อยากเล่นแบบชิล ๆ `
- **Prefix keyword**: `Some("88plus")`
- **Sub-tokens**: `["88", "plus"]`
- **Primary intent**: `Navigational` (confidence: 28%)

| Keyword | Score |
|---------|-------|
| 88plus | 1.00 |
| 88 | 0.80 |
| plus | 0.80 |
| เล่น | 0.60 |
| joker | 0.20 |
| กติกา | 0.20 |

### Sample 23

- **Input**: `99. 2 VIDEOS. ควีน888 ฝาก-ถอนระบบอัตโนมัติ อัปเดตโปรโมชั่นล่าสุด! สล็อตเครดิตไม่จำกัด ทดลองใช้ฟรี ไม`
- **Prefix keyword**: `Some("99")`
- **Sub-tokens**: `["99"]`
- **Primary intent**: `Transactional` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| 99 | 1.00 |
| โปรโมชั่น | 0.40 |
| 888 | 0.40 |
| ฝาก | 0.40 |
| ถอน | 0.40 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |
| สล็อต | 0.20 |

### Sample 24

- **Input**: `About this item. ผล ฟุตบอล สด 888 นี่เพื่อนๆ รู้จักกันไหม? มันคือเว็บที่รวบรวมผลบอลสดแบบเรียลไทม์จาก`
- **Prefix keyword**: `Some("about")`
- **Sub-tokens**: `["about"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| about | 1.00 |
| 888 | 0.20 |

### Sample 25

- **Input**: `After testing, I gotta admit, 888 Casino really holds water when it comes to legit online gambling. `
- **Prefix keyword**: `Some("after")`
- **Sub-tokens**: `["after"]`
- **Primary intent**: `Navigational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| after | 1.00 |
| 888 | 0.20 |
| casino | 0.20 |

### Sample 26

- **Input**: `ALLSLOT MASTER รวม Live Casino ที่ภาพคมชัด ดีลเลอร์จริง ระบบเสียงชัด เล่นได้ทุกที่ เมนูเลือกโต๊ะง่าย`
- **Prefix keyword**: `Some("allslot")`
- **Sub-tokens**: `["all", "slot"]`
- **Primary intent**: `Navigational` (confidence: 79%)

| Keyword | Score |
|---------|-------|
| allslot | 1.00 |
| all | 0.80 |
| slot | 0.80 |
| เล่น | 0.80 |
| casino | 0.20 |
| 888 | 0.20 |

### Sample 27

- **Input**: `ALLSLOTZ เลือก PG Soft ที่จังหวะเล่นสบาย เมนูไม่รก ฟีเจอร์พอดี ทำให้มือใหม่เข้าถึงง่าย และยังมีธีมให`
- **Prefix keyword**: `Some("allslotz")`
- **Sub-tokens**: `["all", "slot"]`
- **Primary intent**: `Navigational` (confidence: 87%)

| Keyword | Score |
|---------|-------|
| allslotz | 1.00 |
| all | 0.80 |
| slot | 0.80 |
| เล่น | 0.40 |

### Sample 28

- **Input**: `AMB168 สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้กา`
- **Prefix keyword**: `Some("amb168")`
- **Sub-tokens**: `["amb", "168"]`
- **Primary intent**: `Informational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| amb168 | 1.00 |
| amb | 0.80 |
| 168 | 0.80 |
| เล่น | 0.60 |
| สล็อต | 0.20 |
| แนะนำ | 0.20 |

### Sample 29

- **Input**: `AMB488 Roullete เหมาะกับคนชอบความตื่นเต้นจากวงล้อ เลือกแนวเดิมพันตามถนัด ตั้งเป้าเป็นรอบ และเล่นอย่า`
- **Prefix keyword**: `Some("amb488")`
- **Sub-tokens**: `["amb", "488"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| amb488 | 1.00 |
| amb | 0.80 |
| 488 | 0.80 |
| เล่น | 0.40 |

### Sample 30

- **Input**: `AMBBET89 LOGIN โฟกัสที่อัตราชนะสูงด้วยการตั้งเป้าหมายเป็นรอบ แบ่งงบให้ชัด รู้จังหวะพัก และเลือกเกมที`
- **Prefix keyword**: `Some("ambbet89")`
- **Sub-tokens**: `["amb", "bet", "89"]`
- **Primary intent**: `Navigational` (confidence: 47%)

| Keyword | Score |
|---------|-------|
| ambbet89 | 1.00 |
| amb | 0.80 |
| bet | 0.80 |
| 89 | 0.80 |
| เล่น | 0.40 |

### Sample 31

- **Input**: `Apr 26, 2559 BE — Berikut adalah beberapa tips untuk memaksimalkan kemenangan Anda saat bermain slot`
- **Prefix keyword**: `Some("apr")`
- **Sub-tokens**: `["apr"]`
- **Primary intent**: `Navigational` (confidence: 75%)

| Keyword | Score |
|---------|-------|
| apr | 1.00 |
| slot | 0.60 |

### Sample 32

- **Input**: `ATM189 คัด PG Soft ที่เล่นแล้วไม่จำเจ ธีมสวย ลูกเล่นพอดี ไม่ซับซ้อนจนเกินไป เล่นได้ทั้งวันแบบสบาย ๆ `
- **Prefix keyword**: `Some("atm189")`
- **Sub-tokens**: `["atm", "189"]`
- **Primary intent**: `Informational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| atm189 | 1.00 |
| atm | 0.80 |
| 189 | 0.80 |
| เล่น | 0.80 |

### Sample 33

- **Input**: `AVA168 คัด PG Soft ที่เล่นแล้วไม่จำเจ ธีมสวย ลูกเล่นพอดี ไม่ซับซ้อนจนเกินไป เล่นได้ทั้งวันแบบสบาย ๆ `
- **Prefix keyword**: `Some("ava168")`
- **Sub-tokens**: `["ava", "168"]`
- **Primary intent**: `Informational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| ava168 | 1.00 |
| ava | 0.80 |
| 168 | 0.80 |
| เล่น | 0.80 |

### Sample 34

- **Input**: `BAANPOLBALL69 แนว Lucky Win ทำให้การเล่นสนุกขึ้นด้วยโปรที่เข้าถึงง่าย เงื่อนไขไม่ซับซ้อน เหมาะกับคนท`
- **Prefix keyword**: `Some("baanpolball69")`
- **Sub-tokens**: `["baan", "pol", "ball", "69"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| baanpolball69 | 1.00 |
| baan | 0.80 |
| pol | 0.80 |
| ball | 0.80 |
| 69 | 0.80 |
| เล่น | 0.40 |
| เงื่อนไข | 0.40 |
| รายละเอียด | 0.20 |

### Sample 35

- **Input**: `BAANPONBALL เริ่ม Baccarat แบบสบาย ๆ ด้วยกติกาที่เข้าใจง่าย เล่นเป็นรอบได้รวดเร็ว เหมาะกับคนมีเวลาน้`
- **Prefix keyword**: `Some("baanponball")`
- **Sub-tokens**: `["baan", "pon", "ball"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| baanponball | 1.00 |
| baan | 0.80 |
| pon | 0.80 |
| ball | 0.80 |
| เล่น | 0.60 |
| กติกา | 0.20 |

### Sample 36

- **Input**: `BBWTHAI RTP API ช่วยให้คุณเห็นข้อมูลประกอบการเลือกเกมได้สะดวกขึ้น ลดการเดาสุ่ม และช่วยให้วางแผนการเล`
- **Prefix keyword**: `Some("bbwthai")`
- **Sub-tokens**: `["bbw", "thai"]`
- **Primary intent**: `Informational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| bbwthai | 1.00 |
| bbw | 0.80 |
| thai | 0.80 |
| เล่น | 0.80 |
| สล็อต | 0.20 |

### Sample 37

- **Input**: `BET5688 LOGIN แนว Lucky Win ทำให้การเล่นสนุกขึ้นด้วยโปรที่เข้าถึงง่าย เงื่อนไขไม่ซับซ้อน เหมาะกับคนท`
- **Prefix keyword**: `Some("bet5688")`
- **Sub-tokens**: `["bet", "568"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| bet5688 | 1.00 |
| bet | 0.80 |
| 568 | 0.80 |
| เงื่อนไข | 0.40 |
| เล่น | 0.40 |
| รายละเอียด | 0.20 |

### Sample 38

- **Input**: `BETFLIXMM โฟกัสที่อัตราชนะสูงด้วยการตั้งเป้าหมายเป็นรอบ แบ่งงบให้ชัด รู้จังหวะพัก และเลือกเกมที่เหมา`
- **Prefix keyword**: `Some("betflixmm")`
- **Sub-tokens**: `["bet", "flix", "mm"]`
- **Primary intent**: `Navigational` (confidence: 47%)

| Keyword | Score |
|---------|-------|
| betflixmm | 1.00 |
| bet | 0.80 |
| flix | 0.80 |
| mm | 0.80 |
| เล่น | 0.40 |

### Sample 39

- **Input**: `BETFLIXPLUS สนุกกับ Roullete ด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่เข้ากับสไตล์คุณได้ง่าย วางแผนเป็นร`
- **Prefix keyword**: `Some("betflixplus")`
- **Sub-tokens**: `["bet", "flix", "plus"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| betflixplus | 1.00 |
| bet | 0.80 |
| flix | 0.80 |
| plus | 0.80 |
| เล่น | 0.20 |

### Sample 40

- **Input**: `BETUFA888 สำหรับคนเล่น สล็อตออนไลน์ เป็นประจำ แนะนำแนวทางเลือกเกมตามงบ ตั้งเวลาเล่นเป็นรอบ ลดการเล่น`
- **Prefix keyword**: `Some("betufa888")`
- **Sub-tokens**: `["bet", "ufa", "888"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| betufa888 | 1.00 |
| bet | 0.80 |
| ufa | 0.80 |
| 888 | 0.80 |
| เล่น | 1.00 |
| แนะนำ | 0.20 |
| สล็อต | 0.20 |

### Sample 41

- **Input**: `BIG1688 เลือก PG Soft ที่จังหวะเล่นสบาย เมนูไม่รก ฟีเจอร์พอดี ทำให้มือใหม่เข้าถึงง่าย และยังมีธีมใหม`
- **Prefix keyword**: `Some("big1688")`
- **Sub-tokens**: `["big", "168"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| big1688 | 1.00 |
| big | 0.80 |
| 168 | 0.80 |
| เล่น | 0.40 |

### Sample 42

- **Input**: `BLOOKET สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้ก`
- **Prefix keyword**: `Some("blooket")`
- **Sub-tokens**: `["look", "et"]`
- **Primary intent**: `Navigational` (confidence: 28%)

| Keyword | Score |
|---------|-------|
| blooket | 1.00 |
| look | 0.80 |
| et | 0.80 |
| เล่น | 0.60 |
| สล็อต | 0.20 |
| แนะนำ | 0.20 |

### Sample 43

- **Input**: `BOBA168 รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่ชอบเกมจั`
- **Prefix keyword**: `Some("boba168")`
- **Sub-tokens**: `["oba", "168"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| boba168 | 1.00 |
| oba | 0.80 |
| 168 | 0.80 |
| เล่น | 0.40 |
| jili | 0.20 |
| slot | 0.20 |

### Sample 44

- **Input**: `BOSS945 สนุกกับ Roullete ด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่เข้ากับสไตล์คุณได้ง่าย วางแผนเป็นรอบ ๆ`
- **Prefix keyword**: `Some("boss945")`
- **Sub-tokens**: `["boss", "945"]`
- **Primary intent**: `Informational` (confidence: 7%)

| Keyword | Score |
|---------|-------|
| boss945 | 1.00 |
| boss | 0.80 |
| 945 | 0.80 |
| เล่น | 0.20 |

### Sample 45

- **Input**: `Buy used: $860.85. Delivery Saturday, April 12 to Thailand. Or fastest delivery Thursday, April 10. `
- **Prefix keyword**: `Some("buy")`
- **Sub-tokens**: `["buy"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| buy | 1.00 |
| 888 | 0.20 |

### Sample 46

- **Input**: `C88 คัด SpadeGaming ที่เล่นลื่นบนมือถือ ธีมเยอะ ลูกเล่นพอดี ทำให้เล่นเพลินและไม่เบื่อ เหมาะกับคนที่ช`
- **Prefix keyword**: `Some("c88")`
- **Sub-tokens**: `["88"]`
- **Primary intent**: `Navigational` (confidence: 36%)

| Keyword | Score |
|---------|-------|
| c88 | 1.00 |
| 88 | 0.80 |
| เล่น | 0.80 |
| spade | 0.20 |

### Sample 47

- **Input**: `CLOVER444 เรียนรู้ Baccarat จากพื้นฐาน วางเดิมพันให้เหมาะกับงบ เล่นเป็นรอบ ๆ และหยุดเมื่อถึงเป้า เพื`
- **Prefix keyword**: `Some("clover444")`
- **Sub-tokens**: `["cl", "over", "444"]`
- **Primary intent**: `Informational` (confidence: 15%)

| Keyword | Score |
|---------|-------|
| clover444 | 1.00 |
| cl | 0.80 |
| over | 0.80 |
| 444 | 0.80 |
| เล่น | 0.60 |

### Sample 48

- **Input**: `COCO555 โฟกัสอัตราชนะสูงด้วยการกำหนดงบต่อรอบ ตั้งเป้ากำไรที่ทำได้จริง และไม่ไล่ตามการเสีย ช่วยให้เล่`
- **Prefix keyword**: `Some("coco555")`
- **Sub-tokens**: `["oco", "555"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| coco555 | 1.00 |
| oco | 0.80 |
| 555 | 0.80 |
| เล่น | 0.40 |

### Sample 49

- **Input**: `COIN8VIP สำหรับคนเล่น สล็อตออนไลน์ เป็นประจำ แนะนำแนวทางเลือกเกมตามงบ ตั้งเวลาเล่นเป็นรอบ ลดการเล่นต`
- **Prefix keyword**: `Some("coin8vip")`
- **Sub-tokens**: `["coin", "vip"]`
- **Primary intent**: `Transactional` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| coin8vip | 1.00 |
| coin | 0.80 |
| vip | 0.80 |
| เล่น | 1.00 |
| แนะนำ | 0.20 |
| สล็อต | 0.20 |

### Sample 50

- **Input**: `Contact us – หลักสูตรสหสาขาวิชาการจัดการสารอันตรายและสิ่งแวดล้อม ... 888 ล่าสุด เว็บโจ๊กเกอร์888 เว็`
- **Prefix keyword**: `Some("contact")`
- **Sub-tokens**: `["contact"]`
- **Primary intent**: `Navigational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| contact | 1.00 |
| 888 | 0.40 |
| ลงทะเบียน | 0.20 |
| ฟรี | 0.20 |

### Sample 51

- **Input**: `CROW168 รวม JILI Slot ที่เล่นลื่น หมุนเร็ว ฟีเจอร์เด่นทำให้ลุ้นได้บ่อย เหมาะกับคนชอบเกมสปีดไวและอยาก`
- **Prefix keyword**: `Some("crow168")`
- **Sub-tokens**: `["crow", "168"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| crow168 | 1.00 |
| crow | 0.80 |
| 168 | 0.80 |
| เล่น | 0.40 |
| jili | 0.20 |
| slot | 0.20 |

### Sample 52

- **Input**: `cs 2 ping test ฝาก 20 รับ 100,แพลตฟอร์มตรงอันดับ 1 ล่าสุด,เกมทำงานได้อย่างราบรื่น,รวมเกมจากผู้ให้บริ`
- **Prefix keyword**: `Some("cs")`
- **Sub-tokens**: `["cs"]`
- **Primary intent**: `Commercial` (confidence: 31%)

| Keyword | Score |
|---------|-------|
| cs | 1.00 |
| โบนัส | 0.20 |
| พิเศษ | 0.20 |
| สมาชิก | 0.20 |
| ufa | 0.20 |
| bet | 0.20 |
| โปรโมชั่น | 0.20 |
| ฝาก | 0.20 |
| สล็อต | 0.20 |

### Sample 53

- **Input**: `demo-slot-gg-soft-bounce-ball-aztec/>. Platform : PlayStation 4. 4.2 4.2 out of 5 stars 394 ratings.`
- **Prefix keyword**: `Some("demo")`
- **Sub-tokens**: `["demo"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| demo | 1.00 |
| slot | 0.20 |

### Sample 54

- **Input**: `demo-slot-gg-soft-bounce-ball-aztec role= role=>Visit the PlayStation Store. Platform : PlayStation `
- **Prefix keyword**: `Some("demo")`
- **Sub-tokens**: `["demo"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| demo | 1.00 |
| slot | 0.20 |

### Sample 55

- **Input**: `DK7VIP รวม PG Slot ที่ภาพสวยและลูกเล่นแน่น ทำให้ลุ้นฟรีสปินได้สนุก เล่นเพลินบนมือถือ และเหมาะกับคนที`
- **Prefix keyword**: `Some("dk7vip")`
- **Sub-tokens**: `["dk", "vip"]`
- **Primary intent**: `Transactional` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| dk7vip | 1.00 |
| dk | 0.80 |
| vip | 0.80 |
| เล่น | 0.60 |
| รายละเอียด | 0.20 |
| ฟรี | 0.20 |
| slot | 0.20 |
| pg slot | 0.20 |

### Sample 56

- **Input**: `DOGGY รวมเกม SpadeGaming ที่ออกแบบมาเล่นง่าย ฟีเจอร์พอดีและภาพสวย ช่วยให้เล่นเพลินบนมือถือ เหมาะกับค`
- **Prefix keyword**: `Some("doggy")`
- **Sub-tokens**: `["dog", "gy"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| doggy | 1.00 |
| dog | 0.80 |
| gy | 0.80 |
| เล่น | 0.60 |
| spade | 0.20 |

### Sample 57

- **Input**: `DOOSERIES4K Baccarat เล่นไม่ซับซ้อน เหมาะกับคนเริ่มใหม่และคนมีเวลาน้อย แนะนำการเล่นเป็นรอบ คุมงบ และ`
- **Prefix keyword**: `Some("dooseries4k")`
- **Sub-tokens**: `["do", "os", "eries"]`
- **Primary intent**: `Informational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| dooseries4k | 1.00 |
| do | 0.80 |
| os | 0.80 |
| eries | 0.80 |
| เล่น | 0.80 |
| แนะนำ | 0.20 |

### Sample 58

- **Input**: `Download the App for the best experience. Shop through our app to enjoy: Exclusive Vouchers; Better `
- **Prefix keyword**: `Some("download")`
- **Sub-tokens**: `["download"]`
- **Primary intent**: `Navigational` (confidence: 44%)

| Keyword | Score |
|---------|-------|
| download | 1.00 |
| 888 | 0.20 |
| bet | 0.20 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 59

- **Input**: `EASY45 รวม PG Slot ธีมหลากหลาย เล่นเพลินบนมือถือ ฟีเจอร์ฟรีสปินและโบนัสทำให้ลุ้นสนุกทุกสปิน เหมาะกับ`
- **Prefix keyword**: `Some("easy45")`
- **Sub-tokens**: `["easy", "45"]`
- **Primary intent**: `Navigational` (confidence: 20%)

| Keyword | Score |
|---------|-------|
| easy45 | 1.00 |
| easy | 0.80 |
| 45 | 0.80 |
| เล่น | 0.60 |
| pg slot | 0.20 |
| ฟรี | 0.20 |
| โบนัส | 0.20 |
| slot | 0.20 |

### Sample 60

- **Input**: `EXP999 เรียนรู้ Baccarat จากพื้นฐาน วางเดิมพันให้เหมาะกับงบ เล่นเป็นรอบ ๆ และหยุดเมื่อถึงเป้า เพื่อใ`
- **Prefix keyword**: `Some("exp999")`
- **Sub-tokens**: `["exp", "999"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| exp999 | 1.00 |
| exp | 0.80 |
| 999 | 0.80 |
| เล่น | 0.60 |

### Sample 61

- **Input**: `EZYPLUS โฟกัสอัตราชนะสูงด้วยการกำหนดงบต่อรอบ ตั้งเป้ากำไรที่ทำได้จริง และไม่ไล่ตามการเสีย ช่วยให้เล่`
- **Prefix keyword**: `Some("ezyplus")`
- **Sub-tokens**: `["zy", "plus"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| ezyplus | 1.00 |
| zy | 0.80 |
| plus | 0.80 |
| เล่น | 0.40 |

### Sample 62

- **Input**: `fafa-212 เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้งสล็อต, รู`
- **Prefix keyword**: `Some("fafa")`
- **Sub-tokens**: `["afa"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| fafa | 1.00 |
| afa | 0.80 |
| เล่น | 0.20 |
| สล็อต | 0.20 |
| รูเล็ต | 0.20 |
| ลงทะเบียน | 0.20 |
| ทดลอง | 0.20 |
| เข้าระบบ | 0.20 |

### Sample 63

- **Input**: `FAFA365 LOGIN รวม สล็อตออนไลน์ ที่คัดเกมเล่นง่าย โบนัสเข้าไว เมนูไม่ซับซ้อน รองรับมือถือทุกค่าย เหมา`
- **Prefix keyword**: `Some("fafa365")`
- **Sub-tokens**: `["afa", "365"]`
- **Primary intent**: `Informational` (confidence: 12%)

| Keyword | Score |
|---------|-------|
| fafa365 | 1.00 |
| afa | 0.80 |
| 365 | 0.80 |
| เล่น | 0.40 |
| โบนัส | 0.20 |
| สล็อต | 0.20 |

### Sample 64

- **Input**: `fafa คาสิโน การสมัครง่าย,เข้าถึงแพลตฟอร์มเกมได้ทันที,เกมแตกง่ายและชนะบ่อย,รวมเกมจากผู้ให้บริการชื่อด`
- **Prefix keyword**: `Some("fafa")`
- **Sub-tokens**: `["afa"]`
- **Primary intent**: `Transactional` (confidence: 38%)

| Keyword | Score |
|---------|-------|
| fafa | 1.00 |
| afa | 0.80 |
| ฝาก | 0.20 |
| สมัคร | 0.20 |
| ถอน | 0.20 |
| ufa | 0.20 |

### Sample 65

- **Input**: `fafa เรามั่นใจในความปลอดภัยและคุณภาพด้วย API ลิขสิทธิ์แท้ อีกทั้งยังมีกิจกรรมรายวันที่อัพเดตอย่างต่อ`
- **Prefix keyword**: `Some("fafa")`
- **Sub-tokens**: `["afa"]`
- **Primary intent**: `Transactional` (confidence: 10%)

| Keyword | Score |
|---------|-------|
| fafa | 1.00 |
| afa | 0.80 |
| สมัคร | 0.20 |

### Sample 66

- **Input**: `fafa เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแคลงทะเบียน คุณจะได้รับประสบการณ์สุดอรรถรสกับหลากหลายเกม`
- **Prefix keyword**: `Some("fafa")`
- **Sub-tokens**: `["afa"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| fafa | 1.00 |
| afa | 0.80 |
| ลงทะเบียน | 0.20 |
| รูเล็ต | 0.20 |
| สล็อต | 0.20 |

### Sample 67

- **Input**: `fafa อยากลองก่อนตัดสินใจใช่ไหม ทดลองเล่นฟรีทุกเกมได้ทันที ไม่มีข้อผูกมัด เพียงคลิกเข้ามา ไม่ว่าคุณจะ`
- **Prefix keyword**: `Some("fafa")`
- **Sub-tokens**: `["afa"]`
- **Primary intent**: `Commercial` (confidence: 20%)

| Keyword | Score |
|---------|-------|
| fafa | 1.00 |
| afa | 0.80 |
| ทดลอง | 0.40 |
| ของขวัญ | 0.20 |
| ลงทะเบียน | 0.20 |
| ฟรี | 0.20 |
| เล่น | 0.20 |

### Sample 68

- **Input**: `FAH89 เรียนรู้ Baccarat จากพื้นฐาน วางเดิมพันให้เหมาะกับงบ เล่นเป็นรอบ ๆ และหยุดเมื่อถึงเป้า เพื่อให`
- **Prefix keyword**: `Some("fah89")`
- **Sub-tokens**: `["ah", "89"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| fah89 | 1.00 |
| ah | 0.80 |
| 89 | 0.80 |
| เล่น | 0.60 |

### Sample 69

- **Input**: `FIWFAN888 เริ่มต้น สล็อตไม่ผ่านเอเย่นต์ แบบไม่ยุ่งยาก ลดขั้นตอนสมัคร เมนูชัดเจน ตรวจสอบได้ง่าย เหมาะ`
- **Prefix keyword**: `Some("fiwfan888")`
- **Sub-tokens**: `["fi", "fan", "888"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| fiwfan888 | 1.00 |
| fi | 0.80 |
| fan | 0.80 |
| 888 | 0.80 |
| สมัคร | 0.20 |
| ขั้นตอน | 0.20 |
| สล็อต | 0.20 |

### Sample 70

- **Input**: `FIZZ169 Lucky Win เน้นโปรที่อ่านเข้าใจง่าย ใช้งานสะดวก ช่วยเพิ่มสีสันในการเล่น เหมาะกับคนที่อยากเล่น`
- **Prefix keyword**: `Some("fizz169")`
- **Sub-tokens**: `["izz", "169"]`
- **Primary intent**: `Informational` (confidence: 35%)

| Keyword | Score |
|---------|-------|
| fizz169 | 1.00 |
| izz | 0.80 |
| 169 | 0.80 |
| เล่น | 1.00 |
| รายละเอียด | 0.20 |
| เงื่อนไข | 0.20 |

### Sample 71

- **Input**: `FKSPIN เลือกเล่น Joker ที่จังหวะเล่นคุ้นมือ เกมไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่ชอบความคลาสสิ`
- **Prefix keyword**: `Some("fkspin")`
- **Sub-tokens**: `["fk", "spin"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| fkspin | 1.00 |
| fk | 0.80 |
| spin | 0.80 |
| เล่น | 0.60 |
| joker | 0.20 |

### Sample 72

- **Input**: `Fulfillment by Amazon (FBA) is a service we offer sellers that lets them store their products in Ama`
- **Prefix keyword**: `Some("fulfillment")`
- **Sub-tokens**: `["ful", "fillment"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| fulfillment | 1.00 |
| ful | 0.80 |
| fillment | 0.80 |

### Sample 73

- **Input**: `FUNNY111 สำหรับคนเล่น สล็อตออนไลน์ เป็นประจำ แนะนำแนวทางเลือกเกมตามงบ ตั้งเวลาเล่นเป็นรอบ ลดการเล่นต`
- **Prefix keyword**: `Some("funny111")`
- **Sub-tokens**: `["fun", "ny", "111"]`
- **Primary intent**: `Informational` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| funny111 | 1.00 |
| fun | 0.80 |
| ny | 0.80 |
| 111 | 0.80 |
| เล่น | 1.00 |
| แนะนำ | 0.20 |
| สล็อต | 0.20 |

### Sample 74

- **Input**: `G2G168D เลือก Slot ที่ออกแบบหน้าเว็บให้ใช้งานสบาย กดเข้าเกมไว ลดการรอคอย เล่นต่อเนื่องได้ดี เหมาะกับ`
- **Prefix keyword**: `Some("g2g168d")`
- **Sub-tokens**: `["168"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| g2g168d | 1.00 |
| 168 | 0.80 |
| เล่น | 0.20 |
| slot | 0.20 |

### Sample 75

- **Input**: `G2G168 สล็อต GG ไม่มียอดทำเทิร์นโอเวอร์ เว็บสล็อตครบวงจร แจกเครดิตฟรี ไม่ต้องฝาก เล่นเกมสล็อตแตกง่าย`
- **Prefix keyword**: `Some("g2g168")`
- **Sub-tokens**: `["168"]`
- **Primary intent**: `Transactional` (confidence: 48%)

| Keyword | Score |
|---------|-------|
| g2g168 | 1.00 |
| 168 | 0.80 |
| สล็อต | 1.00 |
| ฝาก | 0.80 |
| ฟรี | 0.60 |
| ขั้นต่ำ | 0.40 |
| เล่น | 0.40 |
| ไม่ต้องฝาก | 0.40 |
| แจก | 0.20 |
| ฝากเงิน | 0.20 |

### Sample 76

- **Input**: `G2G5B สัมผัส Live Casino กับดีลเลอร์จริง ภาพสดเสียงชัด ระบบลื่นบนมือถือ เหมาะกับคนที่ชอบความเรียลและ`
- **Prefix keyword**: `Some("g2g5b")`
- **Sub-tokens**: `["g2g5b"]`
- **Primary intent**: `Navigational` (confidence: 14%)

| Keyword | Score |
|---------|-------|
| g2g5b | 1.00 |
| เล่น | 0.20 |
| casino | 0.20 |

### Sample 77

- **Input**: `G2GBET789 คัด PG Soft ที่เหมาะกับมือใหม่ เล่นง่ายแต่ยังมีลุ้น โหมดโบนัสอ่านสบาย ทำให้เล่นเพลินได้ยาว`
- **Prefix keyword**: `Some("g2gbet789")`
- **Sub-tokens**: `["gb", "et", "789"]`
- **Primary intent**: `Navigational` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| g2gbet789 | 1.00 |
| gb | 0.80 |
| et | 0.80 |
| 789 | 0.80 |
| เล่น | 0.60 |
| โบนัส | 0.20 |
| bet | 0.20 |

### Sample 78

- **Input**: `G2GOD ระบบออกแบบให้เริ่มเล่น สล็อตไม่ผ่านเอเย่นต์ ได้ทันที ขั้นตอนไม่เยอะ เมนูชัดเจน ลดความยุ่งยาก เ`
- **Prefix keyword**: `Some("g2god")`
- **Sub-tokens**: `["god"]`
- **Primary intent**: `Informational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| g2god | 1.00 |
| god | 0.80 |
| เล่น | 0.40 |
| สล็อต | 0.20 |
| ขั้นตอน | 0.20 |

### Sample 79

- **Input**: `GAMESLOTS249 คัด PG Soft ที่เล่นแล้วไม่จำเจ ธีมสวย ลูกเล่นพอดี ไม่ซับซ้อนจนเกินไป เล่นได้ทั้งวันแบบส`
- **Prefix keyword**: `Some("gameslots249")`
- **Sub-tokens**: `["games", "lots", "249"]`
- **Primary intent**: `Navigational` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| gameslots249 | 1.00 |
| games | 0.80 |
| lots | 0.80 |
| 249 | 0.80 |
| เล่น | 1.00 |
| slot | 0.20 |

### Sample 80

- **Input**: `GAMESLOTS456 Lucky Win เน้นโปรที่อ่านเข้าใจง่าย ใช้งานสะดวก ช่วยเพิ่มสีสันในการเล่น เหมาะกับคนที่อยา`
- **Prefix keyword**: `Some("gameslots456")`
- **Sub-tokens**: `["games", "lots", "456"]`
- **Primary intent**: `Navigational` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| gameslots456 | 1.00 |
| games | 0.80 |
| lots | 0.80 |
| 456 | 0.80 |
| เล่น | 0.40 |
| เงื่อนไข | 0.20 |
| รายละเอียด | 0.20 |
| slot | 0.20 |

### Sample 81

- **Input**: `GCLUB88888 รวม Slot ที่เหมาะกับทั้งคนเล่นสั้น ๆ และคนเล่นยาว เมนูใช้งานง่าย โหลดไว และช่วยให้คุณเปลี`
- **Prefix keyword**: `Some("gclub88888")`
- **Sub-tokens**: `["club", "888", "88"]`
- **Primary intent**: `Navigational` (confidence: 71%)

| Keyword | Score |
|---------|-------|
| gclub88888 | 1.00 |
| club | 0.80 |
| 888 | 0.80 |
| 88 | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |

### Sample 82

- **Input**: `GEISHA888 เลือก Joker ที่เล่นแล้วคุ้นมือ กติกาไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่อยากเล่นแบบชิล`
- **Prefix keyword**: `Some("geisha888")`
- **Sub-tokens**: `["ge", "isha", "888"]`
- **Primary intent**: `Navigational` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| geisha888 | 1.00 |
| ge | 0.80 |
| isha | 0.80 |
| 888 | 0.80 |
| เล่น | 0.60 |
| กติกา | 0.20 |
| joker | 0.20 |

### Sample 83

- **Input**: `gg bet maintenanceยิ่ปีใหม่ 2026 โปรแรง ทดลองฟรี + แต้มต้อนรับ 2026 มาแล้ว! ทดลองใช้ระบบใหม่ รับแต้ม`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ทดลอง | 0.60 |
| ฟรี | 0.40 |
| ฝาก | 0.20 |
| สล็อต | 0.20 |
| เล่น | 0.20 |
| bet | 0.20 |
| ถอน | 0.20 |

### Sample 84

- **Input**: `gg-bet คุณแค่ click เข้าระบบ ก็สามารถทดลองเล่น ส่องข้อมูลฟีเจอร์ใหม่ล่าสุดทุกวันได้แบบเสรี แถมยังมีส`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 31%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| พิเศษ | 0.40 |
| เข้าระบบ | 0.20 |
| ของขวัญ | 0.20 |
| bet | 0.20 |
| ทดลอง | 0.20 |
| เล่น | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 85

- **Input**: `gg betยิ่แชร์คะแนน ร่วมสนุก ไม่มีค่าใช้จ่าย ปี 2026 ล่าสุด ระบบใหม่ ทดลองฟรี ระบบออโต้เร็ว ทดลองวันน`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 56%)

| Keyword | Score |
|---------|-------|
| gg | 0.71 |
| ทดลอง | 1.00 |
| ฟรี | 0.86 |
| เล่น | 0.43 |
| bet | 0.14 |
| โบนัส | 0.14 |
| ขั้นต่ำ | 0.14 |
| รางวัล | 0.14 |

### Sample 86

- **Input**: `gg betยิ่ได้คะแนนทุกตา ฟรีไม่มีหลอก สนใจไหม? ทดลองใช้ฟรีวันนี้ อัปเดตโปรโมชั่นแรงๆ 2026 ระบบใหม่ ฝาก`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 60%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.60 |
| ทดลอง | 0.40 |
| รางวัล | 0.20 |
| ฝาก | 0.20 |
| ถอน | 0.20 |
| โปรโมชั่น | 0.20 |
| bet | 0.20 |

### Sample 87

- **Input**: `gg betยิ่ทดลองเล่นวันนี้ ฟรี ไม่มีขั้นต่ำ 2026 สมัครวันนี้ รับโบนัสต้อนรับฟรี สูตรสล็อต PG 2026 พร้อ`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 47%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.60 |
| เล่น | 0.40 |
| สล็อต | 0.40 |
| ฝาก | 0.40 |
| ทดลอง | 0.40 |
| สมัคร | 0.20 |
| โบนัส | 0.20 |
| bet | 0.20 |

### Sample 88

- **Input**: `gg-games ‍♀️ ฝาก-ถอนตลอด 24 ชั่วโมง,100% ของแท้,มีการฝาก-ถอนโดยไม่มีขั้นต่ำ,รวมเกมจากผู้ให้บริการชื่`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Transactional` (confidence: 75%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฝาก | 0.40 |
| ถอน | 0.40 |
| ขั้นต่ำ | 0.20 |
| 888 | 0.20 |
| slot | 0.20 |

### Sample 89

- **Input**: `gg-gamesยิ่เล่นแล้วได้คะแนนทุกตา แลกรางวัลเพียบ ไม่ต้องเติมเงินเพิ่ม แชร์คะแนน ร่วมสนุก รับของรางวัล`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| รางวัล | 0.40 |
| โบนัส | 0.20 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |
| เล่น | 0.20 |

### Sample 90

- **Input**: `gg-judi id= ระบบสมาชิกที่ออกแบบมาเพื่อคุณ จะมอบสิทธิประโยชน์มากมาย ไม่ว่าจะเป็นของขวัญแต้มสะสม, โปรโ`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| พิเศษ | 0.20 |
| สมาชิก | 0.20 |
| โปรโมชั่น | 0.20 |
| ของขวัญ | 0.20 |

### Sample 91

- **Input**: `gg keystoreยิ่ฟรีไม่มีค่าใช้จ่าย ทดลองระบบใหม่ เริ่มต้นง่ายมาก ทดลองฟรี ไม่ต้องลงทุนอะไรเลย คลิกเลย!`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 46%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.60 |
| ทดลอง | 0.60 |
| เล่น | 0.20 |
| สล็อต | 0.20 |

### Sample 92

- **Input**: `gg keystoreยิ่ร่วมสนุกกับเพื่อน แชร์แต้ม ฟรี 100% 2026 ระบบออโต้ที่ดีที่สุด ปี 2026 ล่าสุด ระบบใหม่ `
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 64%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.60 |
| สมัคร | 0.20 |
| รางวัล | 0.20 |
| ทดลอง | 0.20 |

### Sample 93

- **Input**: `gg keystoreยิ่ร่วมสนุกกับเพื่อน แชร์แต้ม ฟรี 100% ทดลองเล่นวันนี้ ฟรี ไม่มีขั้นต่ำ 2026 นอกจากสล็อตน`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.60 |
| ทดลอง | 0.40 |
| สล็อต | 0.20 |
| ขั้นต่ำ | 0.20 |
| พิเศษ | 0.20 |
| เล่น | 0.20 |

### Sample 94

- **Input**: `gg keystoreยิ่ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ แชร์แต้มกับเพื่อนวันนี้ `
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 27%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| รางวัล | 0.20 |
| เล่น | 0.20 |
| สมาชิก | 0.20 |
| พิเศษ | 0.20 |
| โบนัส | 0.20 |
| ทดลอง | 0.20 |

### Sample 95

- **Input**: `gg poker appยิ่อัปเดตใหญ่ 2026 รับคะแนนฟรีทันที แถมไม่ต้องโหลดอะไร! ปลัตฟอร์มรองรับทั้งมือถือและคอม `
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 55%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.60 |
| ทดลอง | 0.40 |
| เล่น | 0.20 |

### Sample 96

- **Input**: `gg-soft-slot id= id= id= id=>`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| slot | 0.20 |

### Sample 97

- **Input**: `gg-soft-slot id= role= role=>. Platform : PlayStation 4. 4.2 4.2 out of 5 stars 394 ratings. $985.88`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| slot | 0.20 |

### Sample 98

- **Input**: `gg-soft-slot id= role= role= role= role= role= role= role= ไม่จำเป็นต้องฝากเงินก่อน แค่เข้าระบบ ก็สา`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Transactional` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| slot | 0.20 |
| เข้าระบบ | 0.20 |
| ฝาก | 0.20 |
| เล่น | 0.20 |
| ฝากเงิน | 0.20 |
| ทดลอง | 0.20 |

### Sample 99

- **Input**: `gg store game ‍♀️ ไม่ผ่านตัวแทน,ฝาก-ถอนตลอด 24 ชั่วโมง,100% ของแท้,รวมเกมจากผู้ให้บริการชื่อดังทั้งห`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Navigational` (confidence: 36%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| bet | 0.60 |
| 888 | 0.20 |
| ฝาก | 0.20 |
| ถอน | 0.20 |

### Sample 100

- **Input**: `gg-คืออะไร ระบบสมาชิกของเราออกแบบมาเพื่อตอบสนองทุกต้องการ ไม่เพียงเท่านั้น ยังมีโปรโมชั่น รางวัลพิเศ`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| โปรโมชั่น | 0.20 |
| สมาชิก | 0.20 |
| รางวัล | 0.20 |
| พิเศษ | 0.20 |
| คืออะไร | 0.20 |

### Sample 101

- **Input**: `gg คืออะไร ระบบสมาชิกถูกออกแบบมาอย่างพิถีพิถัน เพื่อให้คุณพบบสิทธิประโยชน์แบบเต็มๆ ไม่ว่าจะเป็นการสะ`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| สมาชิก | 0.20 |
| คืออะไร | 0.20 |
| โปรโมชั่น | 0.20 |
| ของขวัญ | 0.20 |

### Sample 102

- **Input**: `gg คืออะไร เราออกแบบระบบสมาชิกเป็นพิเศษเพื่อคุณ เพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะ`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Commercial` (confidence: 36%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| รางวัล | 0.20 |
| สมาชิก | 0.20 |
| คืออะไร | 0.20 |
| ของขวัญ | 0.20 |
| พิเศษ | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 103

- **Input**: `gg-คืออะไร เล่นแบบฟรี ๆ ไม่ต้องโอนเงิน ไม่ต้องดาวน์โหลด แพลตฟอร์มเราใช้งานได้ง่ายทั้งบนมือถือและคอมพ`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Informational` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| ฟรี | 0.20 |
| คืออะไร | 0.20 |
| เล่น | 0.20 |

### Sample 104

- **Input**: `gg-คืออะไร อยากลองก่อนตัดสินใจใช่ไหม ระบบใหม่ล่าสุด พร้อมAPI ลิขสิทธิ์แท้ ยินดีให้คุณทดลองเล่นฟรีทุก`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Informational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| เล่น | 0.40 |
| ของขวัญ | 0.20 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |
| ลงทะเบียน | 0.20 |
| คืออะไร | 0.20 |

### Sample 105

- **Input**: `ggสล็อต คือ เว็บ ตรง ที่ ก้าวล้ำ ครั้งใหญ่ ในปี 2024 ซึ่ง สามารถเข้าถึงได้ ตลอด 24 ชั่วโมง และเป็น ท`
- **Prefix keyword**: `Some("gg")`
- **Sub-tokens**: `["gg"]`
- **Primary intent**: `Navigational` (confidence: 56%)

| Keyword | Score |
|---------|-------|
| gg | 1.00 |
| slot | 0.20 |
| ทางเข้า | 0.20 |
| สล็อต | 0.20 |
| pg slot | 0.20 |

### Sample 106

- **Input**: `GOAT999 คัด PG Soft ที่เหมาะกับมือใหม่ เล่นง่ายแต่ยังมีลุ้น โหมดโบนัสอ่านสบาย ทำให้เล่นเพลินได้ยาว แ`
- **Prefix keyword**: `Some("goat999")`
- **Sub-tokens**: `["go", "at", "999"]`
- **Primary intent**: `Navigational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| goat999 | 1.00 |
| go | 0.80 |
| at | 0.80 |
| 999 | 0.80 |
| เล่น | 0.60 |
| โบนัส | 0.20 |

### Sample 107

- **Input**: `GT99BET LOGIN RTP API ช่วยให้คุณเห็นข้อมูลประกอบการเลือกเกมได้สะดวกขึ้น ลดการเดาสุ่ม และช่วยให้วางแผ`
- **Prefix keyword**: `Some("gt99bet")`
- **Sub-tokens**: `["gt", "99", "bet"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| gt99bet | 1.00 |
| gt | 0.80 |
| 99 | 0.80 |
| bet | 0.80 |
| เล่น | 0.20 |

### Sample 108

- **Input**: `GTR55 ปรับแนวคิดไปที่อัตราชนะสูงด้วยการแบ่งงบเป็นรอบ ตั้งเป้ากำไร-ขาดทุนให้ชัด และพักเมื่อถึงเป้า เพ`
- **Prefix keyword**: `Some("gtr55")`
- **Sub-tokens**: `["tr", "55"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| gtr55 | 1.00 |
| tr | 0.80 |
| 55 | 0.80 |
| เล่น | 0.40 |

### Sample 109

- **Input**: `GUCCI GGウール スカーフの詳細情報. ☆昨年の冬使用。スタイル ‎660025 4G386 41741930年代にデザインされたパターンがベースとなっているGGパターンは、グッチを象徴 ...`
- **Prefix keyword**: `Some("gucci")`
- **Sub-tokens**: `["gu", "cci"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| gucci | 1.00 |
| gu | 0.80 |
| cci | 0.80 |

### Sample 110

- **Input**: `HB777 ใช้ RTP API ดูข้อมูลประกอบการเลือกเกมแบบเรียลไทม์ ช่วยคัดเกมที่เข้ากับงบและจังหวะการเล่น ลดการ`
- **Prefix keyword**: `Some("hb777")`
- **Sub-tokens**: `["hb", "777"]`
- **Primary intent**: `Commercial` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| hb777 | 1.00 |
| hb | 0.80 |
| 777 | 0.80 |
| เล่น | 0.60 |

### Sample 111

- **Input**: `HBD85 Baccarat เล่นไม่ซับซ้อน เหมาะกับคนเริ่มใหม่และคนมีเวลาน้อย แนะนำการเล่นเป็นรอบ คุมงบ และหยุดเม`
- **Prefix keyword**: `Some("hbd85")`
- **Sub-tokens**: `["bd", "85"]`
- **Primary intent**: `Informational` (confidence: 42%)

| Keyword | Score |
|---------|-------|
| hbd85 | 0.63 |
| bd | 0.50 |
| 85 | 0.50 |
| เล่น | 1.00 |
| แนะนำ | 0.25 |
| slot | 0.13 |

### Sample 112

- **Input**: `HELEN168 คัด PG Soft ที่เข้าใจง่าย เล่นได้ต่อเนื่อง ธีมหลากหลาย ฟีเจอร์พอดี ทำให้มือใหม่เริ่มได้ไว แ`
- **Prefix keyword**: `Some("helen168")`
- **Sub-tokens**: `["he", "len", "168"]`
- **Primary intent**: `Informational` (confidence: 11%)

| Keyword | Score |
|---------|-------|
| helen168 | 1.00 |
| he | 0.80 |
| len | 0.80 |
| 168 | 0.80 |
| เล่น | 0.40 |

### Sample 113

- **Input**: `HENG9999 RTP API ช่วยให้คุณเห็นข้อมูลประกอบการเลือกเกมได้สะดวกขึ้น ลดการเดาสุ่ม และช่วยให้วางแผนการเ`
- **Prefix keyword**: `Some("heng9999")`
- **Sub-tokens**: `["heng", "999"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| heng9999 | 1.00 |
| heng | 0.80 |
| 999 | 0.80 |
| เล่น | 0.40 |

### Sample 114

- **Input**: `HIHUAY เรียนรู้ Baccarat จากพื้นฐาน วางเดิมพันให้เหมาะกับงบ เล่นเป็นรอบ ๆ และหยุดเมื่อถึงเป้า เพื่อใ`
- **Prefix keyword**: `Some("hihuay")`
- **Sub-tokens**: `["ihu", "ay"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| hihuay | 1.00 |
| ihu | 0.80 |
| ay | 0.80 |
| เล่น | 0.60 |

### Sample 115

- **Input**: `HISO789 Roullete ช่วยเพิ่มความตื่นเต้นด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่ถนัด ตั้งเป้าเป็นรอบ และเ`
- **Prefix keyword**: `Some("hiso789")`
- **Sub-tokens**: `["iso", "789"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| hiso789 | 1.00 |
| iso | 0.80 |
| 789 | 0.80 |
| เล่น | 0.40 |

### Sample 116

- **Input**: `Home · About IC-KMUTNB · Double Degree Program (3+1 Program) · Academic » · ITBL · ABMS · Admissions`
- **Prefix keyword**: `Some("home")`
- **Sub-tokens**: `["home"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| home | 1.00 |

### Sample 117

- **Input**: `HOTSTAR789 คัด PG Soft ที่เล่นแล้วไม่จำเจ ธีมสวย ลูกเล่นพอดี ไม่ซับซ้อนจนเกินไป เล่นได้ทั้งวันแบบสบา`
- **Prefix keyword**: `Some("hotstar789")`
- **Sub-tokens**: `["hot", "star", "789"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| hotstar789 | 1.00 |
| hot | 0.80 |
| star | 0.80 |
| 789 | 0.80 |
| เล่น | 0.80 |

### Sample 118

- **Input**: `HUAYHIT แนว Lucky Win ทำให้การเล่นสนุกขึ้นด้วยโปรที่เข้าถึงง่าย เงื่อนไขไม่ซับซ้อน เหมาะกับคนที่อยาก`
- **Prefix keyword**: `Some("huayhit")`
- **Sub-tokens**: `["hu", "ay", "hit"]`
- **Primary intent**: `Informational` (confidence: 36%)

| Keyword | Score |
|---------|-------|
| huayhit | 1.00 |
| hu | 0.80 |
| ay | 0.80 |
| hit | 0.80 |
| เล่น | 0.80 |
| เงื่อนไข | 0.80 |
| รายละเอียด | 0.40 |
| ufa | 0.20 |

### Sample 119

- **Input**: `ICONXFUN Roullete ช่วยเพิ่มความตื่นเต้นด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่ถนัด ตั้งเป้าเป็นรอบ และ`
- **Prefix keyword**: `Some("iconxfun")`
- **Sub-tokens**: `["icon", "xf", "un"]`
- **Primary intent**: `Informational` (confidence: 11%)

| Keyword | Score |
|---------|-------|
| iconxfun | 1.00 |
| icon | 0.80 |
| xf | 0.80 |
| un | 0.80 |
| เล่น | 0.40 |

### Sample 120

- **Input**: `idn-gg id= role= อยากสนุกกับเกมคาสิโนโดยไม่ต้องคิดเยอะเลย? เพียงแค่ลงทะเบียน คุณก็จะได้สัมผัสกับหลาก`
- **Prefix keyword**: `Some("idn")`
- **Sub-tokens**: `["id"]`
- **Primary intent**: `Navigational` (confidence: 14%)

| Keyword | Score |
|---------|-------|
| idn | 1.00 |
| id | 0.80 |
| รูเล็ต | 0.20 |
| สล็อต | 0.20 |
| ฟรี | 0.20 |
| ลงทะเบียน | 0.20 |
| เล่น | 0.20 |

### Sample 121

- **Input**: `id test kiss918 login โปรโมชั่นล่าสุด,ฝาก 20 รับ 100,รวมเกมจากผู้ให้บริการชื่อดังทั้งหมดboston777、bo`
- **Prefix keyword**: `Some("id")`
- **Sub-tokens**: `["id"]`
- **Primary intent**: `Commercial` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| id | 1.00 |
| ฝาก | 0.20 |
| โปรโมชั่น | 0.20 |
| 888 | 0.20 |

### Sample 122

- **Input**: `INW99GTR เลือก Joker ที่เล่นแล้วคุ้นมือ กติกาไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่อยากเล่นแบบชิล `
- **Prefix keyword**: `Some("inw99gtr")`
- **Sub-tokens**: `["in", "99", "tr"]`
- **Primary intent**: `Navigational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| inw99gtr | 1.00 |
| in | 0.80 |
| 99 | 0.80 |
| tr | 0.80 |
| เล่น | 0.60 |
| joker | 0.20 |
| กติกา | 0.20 |

### Sample 123

- **Input**: `ITBET คัด SpadeGaming ที่เล่นลื่นบนมือถือ ธีมเยอะ ลูกเล่นพอดี ทำให้เล่นเพลินและไม่เบื่อ เหมาะกับคนที`
- **Prefix keyword**: `Some("itbet")`
- **Sub-tokens**: `["it", "bet"]`
- **Primary intent**: `Navigational` (confidence: 56%)

| Keyword | Score |
|---------|-------|
| itbet | 1.00 |
| it | 0.80 |
| bet | 0.80 |
| เล่น | 0.80 |
| spade | 0.20 |

### Sample 124

- **Input**: `IZE123 เริ่ม Baccarat แบบสบาย ๆ ด้วยกติกาที่เข้าใจง่าย เล่นเป็นรอบได้รวดเร็ว เหมาะกับคนมีเวลาน้อย แล`
- **Prefix keyword**: `Some("ize123")`
- **Sub-tokens**: `["ize", "123"]`
- **Primary intent**: `Informational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| ize123 | 1.00 |
| ize | 0.80 |
| 123 | 0.80 |
| เล่น | 0.60 |
| กติกา | 0.20 |

### Sample 125

- **Input**: `JACKPOT คัด SpadeGaming ที่เล่นลื่นบนมือถือ ธีมเยอะ ลูกเล่นพอดี ทำให้เล่นเพลินและไม่เบื่อ เหมาะกับคน`
- **Prefix keyword**: `Some("jackpot")`
- **Sub-tokens**: `["jack", "pot"]`
- **Primary intent**: `Informational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| jackpot | 1.00 |
| jack | 0.80 |
| pot | 0.80 |
| เล่น | 0.80 |
| spade | 0.20 |

### Sample 126

- **Input**: `Jan 13, 2569 BE — In this short article, we will certainly guide you through the world of online gam`
- **Prefix keyword**: `Some("jan")`
- **Sub-tokens**: `["jan"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| jan | 1.00 |

### Sample 127

- **Input**: `Jan 20, 2569 BE — BECK168 เริ่ม Baccarat แบบสบาย ๆ ด้วยกติกาที่เข้าใจง่าย เล่นเป็นรอบได้รวดเร็ว เหมา`
- **Prefix keyword**: `Some("jan")`
- **Sub-tokens**: `["jan"]`
- **Primary intent**: `Informational` (confidence: 44%)

| Keyword | Score |
|---------|-------|
| jan | 1.00 |
| เล่น | 0.60 |
| กติกา | 0.20 |

### Sample 128

- **Input**: `JOKERSLOT789 สัมผัส Live Casino แบบถ่ายทอดสดจริง เลือกโต๊ะง่าย ระบบลื่นบนมือถือ ทำให้เล่นได้ทุกที่ เ`
- **Prefix keyword**: `Some("jokerslot789")`
- **Sub-tokens**: `["ok", "ers", "lot", "789"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| jokerslot789 | 1.00 |
| ok | 0.80 |
| ers | 0.80 |
| lot | 0.80 |
| 789 | 0.80 |
| joker | 0.20 |
| slot | 0.20 |
| casino | 0.20 |
| เล่น | 0.20 |

### Sample 129

- **Input**: `KC9 OOO 🏋️‍♀️ ควบคุมโต๊ะโป๊กเกอร์ด้วยเทคนิคขั้นสูงที่ช่วยให้คุณได้เปรียบเหนือคู่ต่อสู้ ช่วยให้คุณชนะ`
- **Prefix keyword**: `Some("kc9")`
- **Sub-tokens**: `["kc"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| kc9 | 1.00 |
| kc | 0.80 |

### Sample 130

- **Input**: `KENZO365 รวม PG Slot ที่มีธีมหลากหลายและฟีเจอร์ฟรีสปินน่าลุ้น เล่นลื่นบนมือถือ เหมาะกับคนที่อยากได้เ`
- **Prefix keyword**: `Some("kenzo365")`
- **Sub-tokens**: `["ken", "zo", "365"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| kenzo365 | 1.00 |
| ken | 0.80 |
| zo | 0.80 |
| 365 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| pg slot | 0.20 |
| ฟรี | 0.20 |

### Sample 131

- **Input**: `KHO688 LOGIN เลือกเล่น Slot บนเว็บที่ออกแบบให้กดง่าย โหลดไว ระบบเสถียร เล่นต่อเนื่องไม่สะดุด เหมาะกั`
- **Prefix keyword**: `Some("kho688")`
- **Sub-tokens**: `["ho", "688"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| kho688 | 1.00 |
| ho | 0.80 |
| 688 | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |

### Sample 132

- **Input**: `KING88 รวม Slot ที่เหมาะกับทั้งคนเล่นสั้น ๆ และคนเล่นยาว เมนูใช้งานง่าย โหลดไว และช่วยให้คุณเปลี่ยนเ`
- **Prefix keyword**: `Some("king88")`
- **Sub-tokens**: `["king", "88"]`
- **Primary intent**: `Navigational` (confidence: 35%)

| Keyword | Score |
|---------|-------|
| king88 | 1.00 |
| king | 0.80 |
| 88 | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |

### Sample 133

- **Input**: `KINGKAN569 รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่ชอบเก`
- **Prefix keyword**: `Some("kingkan569")`
- **Sub-tokens**: `["king", "kan", "569"]`
- **Primary intent**: `Navigational` (confidence: 14%)

| Keyword | Score |
|---------|-------|
| kingkan569 | 1.00 |
| king | 0.80 |
| kan | 0.80 |
| 569 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 134

- **Input**: `KOI88 LOGIN รวม JILI Slot ที่เล่นลื่น หมุนเร็ว ฟีเจอร์เด่นทำให้ลุ้นได้บ่อย เหมาะกับคนชอบเกมสปีดไวและ`
- **Prefix keyword**: `Some("koi88")`
- **Sub-tokens**: `["oi", "88"]`
- **Primary intent**: `Navigational` (confidence: 41%)

| Keyword | Score |
|---------|-------|
| koi88 | 1.00 |
| oi | 0.80 |
| 88 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 135

- **Input**: `LAVAGAME99TH LOGIN แนว Lucky Win ทำให้การเล่นสนุกขึ้นด้วยโปรที่เข้าถึงง่าย เงื่อนไขไม่ซับซ้อน เหมาะก`
- **Prefix keyword**: `Some("lavagame99th")`
- **Sub-tokens**: `["lav", "ag", "ame", "99", "th"]`
- **Primary intent**: `Informational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| lavagame99th | 1.00 |
| lav | 0.80 |
| ag | 0.80 |
| ame | 0.80 |
| 99 | 0.80 |
| th | 0.80 |
| เล่น | 0.40 |
| เงื่อนไข | 0.40 |
| รายละเอียด | 0.20 |

### Sample 136

- **Input**: `LINE 222 สล็อต ทางเข้าใหม่ ⚡ ทดลองเล่นสล็อต PGSLOT เว็บตรงแท้ รองรับทรูวอเลท ฝากถอนออโต้ 24 ชั่วโมง.`
- **Prefix keyword**: `Some("line")`
- **Sub-tokens**: `["line"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| line | 1.00 |
| สล็อต | 1.00 |
| เล่น | 0.40 |
| ถอน | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝาก | 0.20 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |
| รูเล็ต | 0.20 |

### Sample 137

- **Input**: `LIVE44 สนุกกับ Roullete ด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่เข้ากับสไตล์คุณได้ง่าย วางแผนเป็นรอบ ๆ `
- **Prefix keyword**: `Some("live44")`
- **Sub-tokens**: `["live", "44"]`
- **Primary intent**: `Navigational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| live44 | 1.00 |
| live | 0.80 |
| 44 | 0.80 |
| เล่น | 0.20 |

### Sample 138

- **Input**: `LOTTERY118 คัด SpadeGaming ที่เล่นลื่นบนมือถือ ธีมเยอะ ลูกเล่นพอดี ทำให้เล่นเพลินและไม่เบื่อ เหมาะกั`
- **Prefix keyword**: `Some("lottery118")`
- **Sub-tokens**: `["lot", "tery", "118"]`
- **Primary intent**: `Navigational` (confidence: 41%)

| Keyword | Score |
|---------|-------|
| lottery118 | 1.00 |
| lot | 0.80 |
| tery | 0.80 |
| 118 | 0.80 |
| เล่น | 0.80 |
| spade | 0.20 |

### Sample 139

- **Input**: `LTOBET รวม PG Slot ธีมหลากหลาย เล่นเพลินบนมือถือ ฟีเจอร์ฟรีสปินและโบนัสทำให้ลุ้นสนุกทุกสปิน เหมาะกับ`
- **Prefix keyword**: `Some("ltobet")`
- **Sub-tokens**: `["lt", "obet"]`
- **Primary intent**: `Navigational` (confidence: 67%)

| Keyword | Score |
|---------|-------|
| ltobet | 1.00 |
| lt | 0.80 |
| obet | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |
| ฟรี | 0.20 |
| bet | 0.20 |
| โบนัส | 0.20 |
| pg slot | 0.20 |

### Sample 140

- **Input**: `LUCIA389 รวม PG Slot ที่ภาพสวยและลูกเล่นแน่น ทำให้ลุ้นฟรีสปินได้สนุก เล่นเพลินบนมือถือ และเหมาะกับคน`
- **Prefix keyword**: `Some("lucia389")`
- **Sub-tokens**: `["luc", "ia", "389"]`
- **Primary intent**: `Navigational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| lucia389 | 0.83 |
| luc | 0.67 |
| ia | 0.67 |
| 389 | 0.67 |
| เล่น | 1.00 |
| slot | 0.33 |
| ufa | 0.17 |
| รายละเอียด | 0.17 |
| ฟรี | 0.17 |
| pg slot | 0.17 |

### Sample 141

- **Input**: `LUCKINBET ด้วย RTP API คุณสามารถดูข้อมูลเกมเพื่อประกอบการเลือกเล่นได้ง่ายขึ้น ช่วยให้ตัดสินใจแบบมีเห`
- **Prefix keyword**: `Some("luckinbet")`
- **Sub-tokens**: `["luck", "in", "bet"]`
- **Primary intent**: `Navigational` (confidence: 68%)

| Keyword | Score |
|---------|-------|
| luckinbet | 1.00 |
| luck | 0.80 |
| in | 0.80 |
| bet | 0.80 |
| เล่น | 0.40 |

### Sample 142

- **Input**: `M33 คัด SpadeGaming ที่เล่นลื่นบนมือถือ ธีมเยอะ ลูกเล่นพอดี ทำให้เล่นเพลินและไม่เบื่อ เหมาะกับคนที่ช`
- **Prefix keyword**: `Some("m33")`
- **Sub-tokens**: `["33"]`
- **Primary intent**: `Informational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| m33 | 1.00 |
| 33 | 0.80 |
| เล่น | 0.80 |
| spade | 0.20 |

### Sample 143

- **Input**: `M66 สไตล์ Lucky Win เน้นโปรที่อ่านแล้วเข้าใจไว เงื่อนไขไม่ซับซ้อน ช่วยเพิ่มความสนุกและความคุ้มค่าในก`
- **Prefix keyword**: `Some("m66")`
- **Sub-tokens**: `["66"]`
- **Primary intent**: `Informational` (confidence: 44%)

| Keyword | Score |
|---------|-------|
| m66 | 1.00 |
| 66 | 0.80 |
| เล่น | 0.80 |
| เงื่อนไข | 0.40 |
| รายละเอียด | 0.20 |

### Sample 144

- **Input**: `M68 รวม PG Slot ที่ภาพสวยและลูกเล่นแน่น ทำให้ลุ้นฟรีสปินได้สนุก เล่นเพลินบนมือถือ และเหมาะกับคนที่อย`
- **Prefix keyword**: `Some("m68")`
- **Sub-tokens**: `["68"]`
- **Primary intent**: `Informational` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| m68 | 1.00 |
| 68 | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |
| ฟรี | 0.20 |
| pg slot | 0.20 |
| รายละเอียด | 0.20 |

### Sample 145

- **Input**: `M918 คัด PG Soft ที่เหมาะกับมือใหม่ เล่นง่ายแต่ยังมีลุ้น โหมดโบนัสอ่านสบาย ทำให้เล่นเพลินได้ยาว และม`
- **Prefix keyword**: `Some("m918")`
- **Sub-tokens**: `["918"]`
- **Primary intent**: `Informational` (confidence: 39%)

| Keyword | Score |
|---------|-------|
| m918 | 0.83 |
| 918 | 0.67 |
| เล่น | 1.00 |
| โบนัส | 0.17 |
| กติกา | 0.17 |
| joker | 0.17 |

### Sample 146

- **Input**: `M99 รวม สล็อตออนไลน์ ที่มีธีมให้เลือกเยอะ เล่นได้ทั้งสายชิลและสายลุย เมนูใช้งานง่าย รองรับมือถือ และ`
- **Prefix keyword**: `Some("m99")`
- **Sub-tokens**: `["99"]`
- **Primary intent**: `Informational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| m99 | 1.00 |
| 99 | 0.80 |
| เล่น | 0.40 |
| สล็อต | 0.20 |

### Sample 147

- **Input**: `MACAU69 สล็อต คาสิโน บาราค่า เดิมพันกีฬา โดย : MACAU69 เมื่อ : 2026-01-30 20:15:20. 0. 0 ...`
- **Prefix keyword**: `Some("macau69")`
- **Sub-tokens**: `["mac", "au", "69"]`
- **Primary intent**: `Navigational` (confidence: 6%)

| Keyword | Score |
|---------|-------|
| macau69 | 1.00 |
| mac | 0.80 |
| au | 0.80 |
| 69 | 0.80 |
| สล็อต | 0.20 |

### Sample 148

- **Input**: `Mar 18, 2569 BE — gg-games การสมัครง่าย,เข้าถึงแพลตฟอร์มเกมได้ทันที,เกมแตกง่ายและชนะบ่อย,รวมเกมจากผู`
- **Prefix keyword**: `Some("mar")`
- **Sub-tokens**: `["mar"]`
- **Primary intent**: `Transactional` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| mar | 1.00 |
| สมัคร | 0.20 |

### Sample 149

- **Input**: `Mar 4, 2569 BE — สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ผล บอล พรีเมียร์ ลีก เมื่อ คืน นี้ ค`
- **Prefix keyword**: `Some("mar")`
- **Sub-tokens**: `["mar"]`
- **Primary intent**: `Navigational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| mar | 1.00 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 150

- **Input**: `MARA038 แนว Lucky Win เน้นความคุ้มค่าในแต่ละวัน เงื่อนไขอ่านเข้าใจง่าย ช่วยให้เล่นสนุกขึ้น และวางแผน`
- **Prefix keyword**: `Some("mara038")`
- **Sub-tokens**: `["ara", "038"]`
- **Primary intent**: `Informational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| mara038 | 1.00 |
| ara | 0.80 |
| 038 | 0.80 |
| เล่น | 0.60 |
| เงื่อนไข | 0.20 |

### Sample 151

- **Input**: `MARINAGAME รวม JILI Slot ที่เล่นลื่น หมุนเร็ว ฟีเจอร์เด่นทำให้ลุ้นได้บ่อย เหมาะกับคนชอบเกมสปีดไวและอ`
- **Prefix keyword**: `Some("marinagame")`
- **Sub-tokens**: `["mar", "in", "ag", "ame"]`
- **Primary intent**: `Navigational` (confidence: 28%)

| Keyword | Score |
|---------|-------|
| marinagame | 1.00 |
| mar | 0.80 |
| in | 0.80 |
| ag | 0.80 |
| ame | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 152

- **Input**: `METRO78 เลือก JILI Slot ที่หมุนไวและไม่หน่วง ฟีเจอร์ทำให้ลุ้นได้บ่อย เหมาะกับคนที่อยากได้เกมสนุกแบบก`
- **Prefix keyword**: `Some("metro78")`
- **Sub-tokens**: `["metro", "78"]`
- **Primary intent**: `Navigational` (confidence: 20%)

| Keyword | Score |
|---------|-------|
| metro78 | 1.00 |
| metro | 0.80 |
| 78 | 0.80 |
| jili | 0.20 |
| slot | 0.20 |

### Sample 153

- **Input**: `MKB MUNKONGBET ใช้ RTP API ดูข้อมูลประกอบการเลือกเกมแบบเรียลไทม์ ช่วยคัดเกมที่เข้ากับงบและจังหวะการเ`
- **Prefix keyword**: `Some("mkb")`
- **Sub-tokens**: `["mk"]`
- **Primary intent**: `Informational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| mkb | 1.00 |
| mk | 0.80 |
| เล่น | 0.60 |
| bet | 0.20 |

### Sample 154

- **Input**: `mkสล็อต888 ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นของขวัญแต้มสะสม`
- **Prefix keyword**: `Some("mk")`
- **Sub-tokens**: `["mk"]`
- **Primary intent**: `Transactional` (confidence: 15%)

| Keyword | Score |
|---------|-------|
| mk | 1.00 |
| เล่น | 0.40 |
| สมัคร | 0.20 |
| สมาชิก | 0.20 |
| โปรโมชั่น | 0.20 |
| 888 | 0.20 |
| ของขวัญ | 0.20 |
| สล็อต | 0.20 |

### Sample 155

- **Input**: `mkสล็อต888 ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นของขวัญแต้มสะสม`
- **Prefix keyword**: `Some("mk")`
- **Sub-tokens**: `["mk"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| mk | 1.00 |
| สล็อต | 0.40 |
| 888 | 0.40 |
| ทดลอง | 0.20 |
| ของขวัญ | 0.20 |
| สมัคร | 0.20 |
| สมาชิก | 0.20 |
| รูเล็ต | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 156

- **Input**: `MNX888 รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่ชอบเกมจัง`
- **Prefix keyword**: `Some("mnx888")`
- **Sub-tokens**: `["mn", "888"]`
- **Primary intent**: `Navigational` (confidence: 71%)

| Keyword | Score |
|---------|-------|
| mnx888 | 1.00 |
| mn | 0.80 |
| 888 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 157

- **Input**: `MOONS777 เล่น สล็อตไม่ผ่านเอเย่นต์ แบบไม่ต้องผ่านคนกลาง เริ่มต้นไว ตรวจสอบได้ทุกขั้นตอน เมนูตรงไปตรง`
- **Prefix keyword**: `Some("moons777")`
- **Sub-tokens**: `["mo", "ons", "777"]`
- **Primary intent**: `Informational` (confidence: 10%)

| Keyword | Score |
|---------|-------|
| moons777 | 1.00 |
| mo | 0.80 |
| ons | 0.80 |
| 777 | 0.80 |
| ขั้นตอน | 0.20 |
| สล็อต | 0.20 |
| เล่น | 0.20 |

### Sample 158

- **Input**: `N88BET รวม สล็อตออนไลน์ ที่คัดเกมเล่นง่าย โบนัสเข้าไว เมนูไม่ซับซ้อน รองรับมือถือทุกค่าย เหมาะทั้งมื`
- **Prefix keyword**: `Some("n88bet")`
- **Sub-tokens**: `["88", "bet"]`
- **Primary intent**: `Navigational` (confidence: 74%)

| Keyword | Score |
|---------|-------|
| n88bet | 1.00 |
| 88 | 0.80 |
| bet | 0.80 |
| เล่น | 0.80 |
| สล็อต | 0.20 |
| โบนัส | 0.20 |

### Sample 159

- **Input**: `NAZI888 รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่ชอบเกมจั`
- **Prefix keyword**: `Some("nazi888")`
- **Sub-tokens**: `["azi", "888"]`
- **Primary intent**: `Navigational` (confidence: 71%)

| Keyword | Score |
|---------|-------|
| nazi888 | 1.00 |
| azi | 0.80 |
| 888 | 0.80 |
| เล่น | 0.40 |
| jili | 0.20 |
| slot | 0.20 |

### Sample 160

- **Input**: `NINJA รวม Joker ที่กติกาไม่ซับซ้อน ฟีเจอร์เข้าใจง่าย เล่นได้เพลิน ๆ เหมาะกับคนที่อยากได้เกมคลาสสิกที`
- **Prefix keyword**: `Some("ninja")`
- **Sub-tokens**: `["inja"]`
- **Primary intent**: `Informational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| ninja | 1.00 |
| inja | 0.80 |
| เล่น | 0.40 |
| กติกา | 0.20 |
| joker | 0.20 |

### Sample 161

- **Input**: `NJAV เล่น สล็อตไม่ผ่านเอเย่นต์ แบบลดขั้นตอนให้สั้นที่สุด เมนูชัดเจน เริ่มได้ทันที เหมาะกับคนที่อยากไ`
- **Prefix keyword**: `Some("njav")`
- **Sub-tokens**: `["nj", "av"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| njav | 1.00 |
| nj | 0.80 |
| av | 0.80 |
| สล็อต | 0.20 |
| ขั้นตอน | 0.20 |
| เล่น | 0.20 |

### Sample 162

- **Input**: `NOVERBET แนว Lucky Win เน้นความคุ้มค่าในแต่ละวัน เงื่อนไขอ่านเข้าใจง่าย ช่วยให้เล่นสนุกขึ้น และวางแผ`
- **Prefix keyword**: `Some("noverbet")`
- **Sub-tokens**: `["nover", "bet"]`
- **Primary intent**: `Navigational` (confidence: 53%)

| Keyword | Score |
|---------|-------|
| noverbet | 1.00 |
| nover | 0.80 |
| bet | 0.80 |
| เล่น | 0.60 |
| เงื่อนไข | 0.20 |

### Sample 163

- **Input**: `NOZZ88 ระบบออกแบบให้เริ่มเล่น สล็อตไม่ผ่านเอเย่นต์ ได้ทันที ขั้นตอนไม่เยอะ เมนูชัดเจน ลดความยุ่งยาก `
- **Prefix keyword**: `Some("nozz88")`
- **Sub-tokens**: `["no", "zz", "88"]`
- **Primary intent**: `Navigational` (confidence: 43%)

| Keyword | Score |
|---------|-------|
| nozz88 | 1.00 |
| no | 0.80 |
| zz | 0.80 |
| 88 | 0.80 |
| เล่น | 0.40 |
| ขั้นตอน | 0.20 |
| สล็อต | 0.20 |

### Sample 164

- **Input**: `O888 最新 Windows11 Corei7 動作良好SSDノートパソコン. マイストア在庫： 3844. 税込. 25,100円. カートに入れる.`
- **Prefix keyword**: `Some("o888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| o888 | 1.00 |
| 888 | 0.80 |

### Sample 165

- **Input**: `Only 1 left in stock - order soon. About this item. หวยเวียดนาม>Visit the PlayStation Store. › See m`
- **Prefix keyword**: `Some("only")`
- **Sub-tokens**: `["only"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| only | 1.00 |

### Sample 166

- **Input**: `Only 8 left and in 20+ carts · Sale ends on September 30 · Local taxes included (where applicable) .`
- **Prefix keyword**: `Some("only")`
- **Sub-tokens**: `["only"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| only | 1.00 |

### Sample 167

- **Input**: `Only 8 left and in 20+ carts · Sale ends on September 30 · Local taxes included (where applicable) .`
- **Prefix keyword**: `Some("only")`
- **Sub-tokens**: `["only"]`
- **Primary intent**: `Informational` (confidence: 27%)

| Keyword | Score |
|---------|-------|
| only | 1.00 |
| เล่น | 0.60 |
| bet | 0.20 |
| โบนัส | 0.20 |
| ufa | 0.20 |

### Sample 168

- **Input**: `Only 8 left and in 20+ carts · Sale ends on September 30 · Local taxes included (where applicable) .`
- **Prefix keyword**: `Some("only")`
- **Sub-tokens**: `["only"]`
- **Primary intent**: `Informational` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| only | 1.00 |
| เล่น | 0.40 |
| casino | 0.20 |

### Sample 169

- **Input**: `P2VVIP แนว Lucky Win ทำให้การเล่นสนุกขึ้นด้วยโปรที่เข้าถึงง่าย เงื่อนไขไม่ซับซ้อน เหมาะกับคนที่อยากไ`
- **Prefix keyword**: `Some("p2vvip")`
- **Sub-tokens**: `["vv", "ip"]`
- **Primary intent**: `Transactional` (confidence: 53%)

| Keyword | Score |
|---------|-------|
| p2vvip | 1.00 |
| vv | 0.80 |
| ip | 0.80 |
| เงื่อนไข | 0.40 |
| เล่น | 0.40 |
| รายละเอียด | 0.20 |
| vip | 0.20 |

### Sample 170

- **Input**: `PANAMA888 Baccarat เล่นไม่ซับซ้อน เหมาะกับคนเริ่มใหม่และคนมีเวลาน้อย แนะนำการเล่นเป็นรอบ คุมงบ และหย`
- **Prefix keyword**: `Some("panama888")`
- **Sub-tokens**: `["pan", "ama", "888"]`
- **Primary intent**: `Navigational` (confidence: 41%)

| Keyword | Score |
|---------|-------|
| panama888 | 1.00 |
| pan | 0.80 |
| ama | 0.80 |
| 888 | 0.80 |
| เล่น | 0.80 |
| แนะนำ | 0.20 |

### Sample 171

- **Input**: `pandawa4d เริ่มต้นความสนุกแบบไม่ต้องคิดอะไรเยอะ เพียงแค่ลงทะเบียน สนุกกับเกมหลากหลายทั้งสล็อต PG, รู`
- **Prefix keyword**: `Some("pandawa4d")`
- **Sub-tokens**: `["pand", "awa"]`
- **Primary intent**: `Navigational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| pandawa4d | 1.00 |
| pand | 0.80 |
| awa | 0.80 |
| ลงทะเบียน | 0.20 |
| สล็อต | 0.20 |
| รูเล็ต | 0.20 |

### Sample 172

- **Input**: `PANGPANG รวม PG Slot ที่มีธีมหลากหลายและฟีเจอร์ฟรีสปินน่าลุ้น เล่นลื่นบนมือถือ เหมาะกับคนที่อยากได้เ`
- **Prefix keyword**: `Some("pangpang")`
- **Sub-tokens**: `["pang", "pang"]`
- **Primary intent**: `Navigational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| pangpang | 1.00 |
| pang | 0.80 |
| เล่น | 0.40 |
| ฟรี | 0.20 |
| slot | 0.20 |
| pg slot | 0.20 |

### Sample 173

- **Input**: `PARIS666 LOGIN ระบบออกแบบให้เริ่มเล่น สล็อตไม่ผ่านเอเย่นต์ ได้ทันที ขั้นตอนไม่เยอะ เมนูชัดเจน ลดความ`
- **Prefix keyword**: `Some("paris666")`
- **Sub-tokens**: `["par", "is", "666"]`
- **Primary intent**: `Informational` (confidence: 14%)

| Keyword | Score |
|---------|-------|
| paris666 | 1.00 |
| par | 0.80 |
| is | 0.80 |
| 666 | 0.80 |
| เล่น | 0.40 |
| สล็อต | 0.20 |
| ขั้นตอน | 0.20 |

### Sample 174

- **Input**: `PGBKK Roullete เหมาะกับคนชอบความตื่นเต้นจากวงล้อ เลือกแนวเดิมพันตามถนัด ตั้งเป้าเป็นรอบ และเล่นอย่าง`
- **Prefix keyword**: `Some("pgbkk")`
- **Sub-tokens**: `["pg", "kk"]`
- **Primary intent**: `Navigational` (confidence: 27%)

| Keyword | Score |
|---------|-------|
| pgbkk | 1.00 |
| pg | 0.80 |
| kk | 0.80 |
| เล่น | 0.40 |

### Sample 175

- **Input**: `PGCC รวม Joker ที่กติกาไม่ซับซ้อน ฟีเจอร์เข้าใจง่าย เล่นได้เพลิน ๆ เหมาะกับคนที่อยากได้เกมคลาสสิกที่`
- **Prefix keyword**: `Some("pgcc")`
- **Sub-tokens**: `["pg", "cc"]`
- **Primary intent**: `Navigational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| pgcc | 1.00 |
| pg | 0.80 |
| cc | 0.80 |
| เล่น | 0.40 |
| กติกา | 0.20 |
| joker | 0.20 |

### Sample 176

- **Input**: `PGRICH88 รวม SpadeGaming ที่เล่นลื่นและมีลูกเล่นพอดี ธีมหลากหลายทำให้ไม่เบื่อ เหมาะกับคนที่ชอบสลับเก`
- **Prefix keyword**: `Some("pgrich88")`
- **Sub-tokens**: `["gr", "ich", "88"]`
- **Primary intent**: `Navigational` (confidence: 35%)

| Keyword | Score |
|---------|-------|
| pgrich88 | 1.00 |
| gr | 0.80 |
| ich | 0.80 |
| 88 | 0.80 |
| เล่น | 1.00 |
| spade | 0.20 |
| slot | 0.20 |
| ฟรี | 0.20 |
| pg slot | 0.20 |

### Sample 177

- **Input**: `PG SLOT · Registry · Customer Service · Gift Cards · Sell · Disability Customer ... toto12 id= role=`
- **Prefix keyword**: `Some("pg")`
- **Sub-tokens**: `["pg"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| pg | 1.00 |
| pg slot | 0.20 |
| slot | 0.20 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |

### Sample 178

- **Input**: `PINGPONG88 รวมเกม SpadeGaming ที่ออกแบบมาเล่นง่าย ฟีเจอร์พอดีและภาพสวย ช่วยให้เล่นเพลินบนมือถือ เหมา`
- **Prefix keyword**: `Some("pingpong88")`
- **Sub-tokens**: `["ping", "pong", "88"]`
- **Primary intent**: `Navigational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| pingpong88 | 1.00 |
| ping | 0.80 |
| pong | 0.80 |
| 88 | 0.80 |
| เล่น | 0.60 |
| spade | 0.20 |

### Sample 179

- **Input**: `【Portfolio Diversification】✌️Invest with confidence using AI insights. Start at $100 and watch your `
- **Prefix keyword**: `Some("portfolio")`
- **Sub-tokens**: `["portfolio"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| portfolio | 1.00 |

### Sample 180

- **Input**: `Product Dimensions, 14.3 x 4.5 x 17 inches; 7.41 ounces. Type of item, Video Game. Item model number`
- **Prefix keyword**: `Some("product")`
- **Sub-tokens**: `["product"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| product | 1.00 |
| ufa | 0.20 |

### Sample 181

- **Input**: `Product information ; 4.2 4.2 out of 5 stars 394 ratings. 4.2 out of 5 stars · #83,477 in Video Game`
- **Prefix keyword**: `Some("product")`
- **Sub-tokens**: `["product"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| product | 1.00 |

### Sample 182

- **Input**: `Product information ; 4.2 4.2 out of 5 stars 394 ratings. 4.2 out of 5 stars · #83,477 in Video Game`
- **Prefix keyword**: `Some("product")`
- **Sub-tokens**: `["product"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| product | 1.00 |
| slot | 0.20 |

### Sample 183

- **Input**: `Product information ; 4.2 4.2 out of 5 stars 394 ratings. 4.2 out of 5 stars · #83,477 in Video Game`
- **Prefix keyword**: `Some("product")`
- **Sub-tokens**: `["product"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| product | 1.00 |

### Sample 184

- **Input**: `QQ8 คัด PG Slot ที่ธีมสวยและลูกเล่นหลากหลาย เล่นฟรีสปินทำให้ลุ้นสนุกต่อเนื่อง เหมาะกับคนที่ชอบเกมคุณ`
- **Prefix keyword**: `Some("qq8")`
- **Sub-tokens**: `["qq"]`
- **Primary intent**: `Navigational` (confidence: 27%)

| Keyword | Score |
|---------|-------|
| qq8 | 1.00 |
| qq | 0.80 |
| เล่น | 0.60 |
| pg slot | 0.20 |
| slot | 0.20 |
| ฟรี | 0.20 |

### Sample 185

- **Input**: `RAMA33 ใช้ RTP API ดูข้อมูลประกอบการเลือกเกมแบบเรียลไทม์ ช่วยคัดเกมที่เข้ากับงบและจังหวะการเล่น ลดกา`
- **Prefix keyword**: `Some("rama33")`
- **Sub-tokens**: `["rama", "33"]`
- **Primary intent**: `Informational` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| rama33 | 1.00 |
| rama | 0.80 |
| 33 | 0.80 |
| เล่น | 0.60 |

### Sample 186

- **Input**: `RICHES777ALL เลือก PG Soft ที่จังหวะเล่นสบาย เมนูไม่รก ฟีเจอร์พอดี ทำให้มือใหม่เข้าถึงง่าย และยังมีธ`
- **Prefix keyword**: `Some("riches777all")`
- **Sub-tokens**: `["rich", "es", "777", "all"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| riches777all | 1.00 |
| rich | 0.80 |
| es | 0.80 |
| 777 | 0.80 |
| all | 0.80 |
| เล่น | 0.40 |

### Sample 187

- **Input**: `RM6 เลือกเล่น Slot บนเว็บที่ออกแบบให้กดง่าย โหลดไว ระบบเสถียร เล่นต่อเนื่องไม่สะดุด เหมาะกับคนชอบควา`
- **Prefix keyword**: `Some("rm6")`
- **Sub-tokens**: `["rm"]`
- **Primary intent**: `Informational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| rm6 | 1.00 |
| rm | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |

### Sample 188

- **Input**: `ROAR66 สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้กา`
- **Prefix keyword**: `Some("roar66")`
- **Sub-tokens**: `["ro", "ar", "66"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| roar66 | 1.00 |
| ro | 0.80 |
| ar | 0.80 |
| 66 | 0.80 |
| เล่น | 0.60 |
| แนะนำ | 0.20 |
| สล็อต | 0.20 |

### Sample 189

- **Input**: `ROMAN237 อัตราชนะสูงไม่ได้มาจากดวงอย่างเดียว ตั้งเป้าต่อรอบ แบ่งงบให้ชัด รู้จังหวะพัก และหยุดเมื่อถึ`
- **Prefix keyword**: `Some("roman237")`
- **Sub-tokens**: `["roman", "237"]`
- **Primary intent**: `Informational` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| roman237 | 1.00 |
| roman | 0.80 |
| 237 | 0.80 |
| เล่น | 0.40 |

### Sample 190

- **Input**: `RULED8 รวม PG Slot ที่มีธีมหลากหลายและฟีเจอร์ฟรีสปินน่าลุ้น เล่นลื่นบนมือถือ เหมาะกับคนที่อยากได้เกม`
- **Prefix keyword**: `Some("ruled8")`
- **Sub-tokens**: `["ru", "led"]`
- **Primary intent**: `Navigational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| ruled8 | 1.00 |
| ru | 0.80 |
| led | 0.80 |
| เล่น | 0.40 |
| ฟรี | 0.20 |
| slot | 0.20 |
| pg slot | 0.20 |

### Sample 191

- **Input**: `SABAI388 สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้`
- **Prefix keyword**: `Some("sabai388")`
- **Sub-tokens**: `["abai", "388"]`
- **Primary intent**: `Informational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| sabai388 | 1.00 |
| abai | 0.80 |
| 388 | 0.80 |
| เล่น | 0.60 |
| แนะนำ | 0.20 |
| สล็อต | 0.20 |

### Sample 192

- **Input**: `SAWAN999 รวม PG Slot ธีมหลากหลาย เล่นเพลินบนมือถือ ฟีเจอร์ฟรีสปินและโบนัสทำให้ลุ้นสนุกทุกสปิน เหมาะก`
- **Prefix keyword**: `Some("sawan999")`
- **Sub-tokens**: `["awan", "999"]`
- **Primary intent**: `Navigational` (confidence: 20%)

| Keyword | Score |
|---------|-------|
| sawan999 | 1.00 |
| awan | 0.80 |
| 999 | 0.80 |
| เล่น | 0.60 |
| โบนัส | 0.20 |
| pg slot | 0.20 |
| slot | 0.20 |
| ฟรี | 0.20 |

### Sample 193

- **Input**: `SBO55 รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่ชอบเกมจังห`
- **Prefix keyword**: `Some("sbo55")`
- **Sub-tokens**: `["bo", "55"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| sbo55 | 1.00 |
| bo | 0.80 |
| 55 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 194

- **Input**: `SBR88 อัตราชนะสูงไม่ได้มาจากดวงอย่างเดียว ตั้งเป้าต่อรอบ แบ่งงบให้ชัด รู้จังหวะพัก และหยุดเมื่อถึงเป`
- **Prefix keyword**: `Some("sbr88")`
- **Sub-tokens**: `["br", "88"]`
- **Primary intent**: `Navigational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| sbr88 | 1.00 |
| br | 0.80 |
| 88 | 0.80 |
| เล่น | 0.80 |

### Sample 195

- **Input**: `SBT88 รวมเกม SpadeGaming ที่ออกแบบมาเล่นง่าย ฟีเจอร์พอดีและภาพสวย ช่วยให้เล่นเพลินบนมือถือ เหมาะกับค`
- **Prefix keyword**: `Some("sbt88")`
- **Sub-tokens**: `["bt", "88"]`
- **Primary intent**: `Navigational` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| sbt88 | 1.00 |
| bt | 0.80 |
| 88 | 0.80 |
| เล่น | 0.60 |
| spade | 0.20 |

### Sample 196

- **Input**: `Shipping and return policies ... Order today to get by ... within 30 days. Cost to ship: THB 1.00. S`
- **Prefix keyword**: `Some("shipping")`
- **Sub-tokens**: `["shipping"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| shipping | 1.00 |

### Sample 197

- **Input**: `SKYLINE888 LOGIN รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที`
- **Prefix keyword**: `Some("skyline888")`
- **Sub-tokens**: `["sky", "line", "888"]`
- **Primary intent**: `Navigational` (confidence: 57%)

| Keyword | Score |
|---------|-------|
| skyline888 | 1.00 |
| sky | 0.80 |
| line | 0.80 |
| 888 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 198

- **Input**: `sky test 888 เข้าถึงแพลตฟอร์มเกมได้ทันที,เกมแตกง่ายและชนะบ่อย,รวมเกมจากผู้ให้บริการชื่อดังทั้งหมดVib`
- **Prefix keyword**: `Some("sky")`
- **Sub-tokens**: `["sky"]`
- **Primary intent**: `Navigational` (confidence: 44%)

| Keyword | Score |
|---------|-------|
| sky | 1.00 |
| bet | 0.60 |
| 888 | 0.20 |

### Sample 199

- **Input**: `SLM99 สนุกกับ Roullete ด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่เข้ากับสไตล์คุณได้ง่าย วางแผนเป็นรอบ ๆ เ`
- **Prefix keyword**: `Some("slm99")`
- **Sub-tokens**: `["sl", "99"]`
- **Primary intent**: `Navigational` (confidence: 57%)

| Keyword | Score |
|---------|-------|
| slm99 | 1.00 |
| sl | 0.80 |
| 99 | 0.80 |
| เล่น | 0.20 |

### Sample 200

- **Input**: `SLOT191 เลือก Joker ที่เล่นแล้วคุ้นมือ กติกาไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่อยากเล่นแบบชิล ๆ`
- **Prefix keyword**: `Some("slot191")`
- **Sub-tokens**: `["slot", "191"]`
- **Primary intent**: `Navigational` (confidence: 78%)

| Keyword | Score |
|---------|-------|
| slot191 | 1.00 |
| slot | 0.80 |
| 191 | 0.80 |
| เล่น | 0.60 |
| joker | 0.20 |
| กติกา | 0.20 |

### Sample 201

- **Input**: `Sony Computer Entertainment. Batteries, 2 AA batteries required. (included). Date First Available, M`
- **Prefix keyword**: `Some("sony")`
- **Sub-tokens**: `["sony"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| sony | 1.00 |

### Sample 202

- **Input**: `SPICY99 เลือก PG Soft ที่จังหวะเล่นสบาย เมนูไม่รก ฟีเจอร์พอดี ทำให้มือใหม่เข้าถึงง่าย และยังมีธีมใหม`
- **Prefix keyword**: `Some("spicy99")`
- **Sub-tokens**: `["sp", "icy", "99"]`
- **Primary intent**: `Navigational` (confidence: 21%)

| Keyword | Score |
|---------|-------|
| spicy99 | 1.00 |
| sp | 0.80 |
| icy | 0.80 |
| 99 | 0.80 |
| เล่น | 0.40 |

### Sample 203

- **Input**: `Star Seller. This seller consistently earned 5-star reviews, shipped on time, and replied quickly to`
- **Prefix keyword**: `Some("star")`
- **Sub-tokens**: `["star"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| star | 1.00 |

### Sample 204

- **Input**: `Star Seller. This seller consistently earned 5-star reviews, shipped on time, and replied quickly to`
- **Prefix keyword**: `Some("star")`
- **Sub-tokens**: `["star"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| star | 1.00 |

### Sample 205

- **Input**: `Star Seller. This seller consistently earned 5-star reviews, shipped on time, and replied quickly to`
- **Prefix keyword**: `Some("star")`
- **Sub-tokens**: `["star"]`
- **Primary intent**: `Navigational` (confidence: 40%)

| Keyword | Score |
|---------|-------|
| star | 1.00 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |
| 888 | 0.20 |

### Sample 206

- **Input**: `SURGAPLAY เลือก Joker ที่เล่นแล้วคุ้นมือ กติกาไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่อยากเล่นแบบชิล`
- **Prefix keyword**: `Some("surgaplay")`
- **Sub-tokens**: `["urg", "ap", "lay"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| surgaplay | 1.00 |
| urg | 0.80 |
| ap | 0.80 |
| lay | 0.80 |
| เล่น | 0.60 |
| กติกา | 0.20 |
| joker | 0.20 |

### Sample 207

- **Input**: `teddy168 เว็บพนัน slot เข้าสู่ระบบง่าย ฝากถอนไม่มีขั้นต่ำ, teddy168 [26 ม.ค. 2569], 1, 0. 3430, WINB`
- **Prefix keyword**: `Some("teddy168")`
- **Sub-tokens**: `["eddy", "168"]`
- **Primary intent**: `Transactional` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| teddy168 | 1.00 |
| eddy | 0.80 |
| 168 | 0.80 |
| ฝาก | 0.40 |
| ถอน | 0.40 |
| ทางเข้า | 0.20 |
| ขั้นต่ำ | 0.20 |
| slot | 0.20 |
| เข้าสู่ระบบ | 0.20 |
| สล็อต | 0.20 |

### Sample 208

- **Input**: `TGA878 รวม Slot ที่เหมาะกับทั้งคนเล่นสั้น ๆ และคนเล่นยาว เมนูใช้งานง่าย โหลดไว และช่วยให้คุณเปลี่ยนเ`
- **Prefix keyword**: `Some("tga878")`
- **Sub-tokens**: `["ga", "878"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| tga878 | 1.00 |
| ga | 0.80 |
| 878 | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |

### Sample 209

- **Input**: `THANOS999 ใช้ RTP API ดูข้อมูลประกอบการเลือกเกมแบบเรียลไทม์ ช่วยคัดเกมที่เข้ากับงบและจังหวะการเล่น ล`
- **Prefix keyword**: `Some("thanos999")`
- **Sub-tokens**: `["th", "anos", "999"]`
- **Primary intent**: `Informational` (confidence: 15%)

| Keyword | Score |
|---------|-------|
| thanos999 | 1.00 |
| th | 0.80 |
| anos | 0.80 |
| 999 | 0.80 |
| เล่น | 0.60 |

### Sample 210

- **Input**: `The Best Free Bet Bonus in Zambia · Online Slots Ideal Payout: Maximizing Your Earnings · Opening th`
- **Prefix keyword**: `Some("the")`
- **Sub-tokens**: `["the"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| the | 1.00 |
| casino | 0.20 |
| slot | 0.20 |
| bet | 0.20 |

### Sample 211

- **Input**: `The first thing you should do is contact the seller directly. If you've already done that, your item`
- **Prefix keyword**: `Some("the")`
- **Sub-tokens**: `["the"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| the | 1.00 |

### Sample 212

- **Input**: `THESTAR289 LOGIN รวม Live Casino ที่ภาพคมชัด ดีลเลอร์จริง ระบบเสียงชัด เล่นได้ทุกที่ เมนูเลือกโต๊ะง่`
- **Prefix keyword**: `Some("thestar289")`
- **Sub-tokens**: `["th", "estar", "289"]`
- **Primary intent**: `Informational` (confidence: 10%)

| Keyword | Score |
|---------|-------|
| thestar289 | 1.00 |
| th | 0.80 |
| estar | 0.80 |
| 289 | 0.80 |
| เล่น | 0.40 |
| casino | 0.20 |

### Sample 213

- **Input**: `TKBKING รวม Slot ที่เหมาะกับทั้งคนเล่นสั้น ๆ และคนเล่นยาว เมนูใช้งานง่าย โหลดไว และช่วยให้คุณเปลี่ยน`
- **Prefix keyword**: `Some("tkbking")`
- **Sub-tokens**: `["tk", "king"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| tkbking | 1.00 |
| tk | 0.80 |
| king | 0.80 |
| เล่น | 0.60 |
| slot | 0.20 |

### Sample 214

- **Input**: `TOPONE777 คัด PG Slot ที่ธีมสวยและลูกเล่นหลากหลาย เล่นฟรีสปินทำให้ลุ้นสนุกต่อเนื่อง เหมาะกับคนที่ชอบ`
- **Prefix keyword**: `Some("topone777")`
- **Sub-tokens**: `["top", "one", "777"]`
- **Primary intent**: `Navigational` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| topone777 | 1.00 |
| top | 0.80 |
| one | 0.80 |
| 777 | 0.80 |
| เล่น | 0.60 |
| pg slot | 0.20 |
| slot | 0.20 |
| ฟรี | 0.20 |

### Sample 215

- **Input**: `TV2 Roullete ช่วยเพิ่มความตื่นเต้นด้วยรูปแบบเดิมพันหลากหลาย เลือกแนวที่ถนัด ตั้งเป้าเป็นรอบ และเล่นอ`
- **Prefix keyword**: `Some("tv2")`
- **Sub-tokens**: `["tv"]`
- **Primary intent**: `Informational` (confidence: 41%)

| Keyword | Score |
|---------|-------|
| tv2 | 1.00 |
| tv | 0.80 |
| เล่น | 0.80 |
| เงื่อนไข | 0.40 |
| bet | 0.20 |
| รายละเอียด | 0.20 |

### Sample 216

- **Input**: `UFA1919 LOGIN รวม JILI Slot ที่เล่นลื่น หมุนเร็ว ฟีเจอร์เด่นทำให้ลุ้นได้บ่อย เหมาะกับคนชอบเกมสปีดไวแ`
- **Prefix keyword**: `Some("ufa1919")`
- **Sub-tokens**: `["ufa", "191"]`
- **Primary intent**: `Navigational` (confidence: 71%)

| Keyword | Score |
|---------|-------|
| ufa1919 | 1.00 |
| ufa | 0.80 |
| 191 | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| jili | 0.20 |

### Sample 217

- **Input**: `UFA3366WIN รวม PG Slot ที่มีธีมหลากหลายและฟีเจอร์ฟรีสปินน่าลุ้น เล่นลื่นบนมือถือ เหมาะกับคนที่อยากได`
- **Prefix keyword**: `Some("ufa3366win")`
- **Sub-tokens**: `["ufa", "336", "win"]`
- **Primary intent**: `Navigational` (confidence: 59%)

| Keyword | Score |
|---------|-------|
| ufa3366win | 1.00 |
| ufa | 0.80 |
| 336 | 0.80 |
| win | 0.80 |
| เล่น | 0.40 |
| ฟรี | 0.20 |
| pg slot | 0.20 |
| slot | 0.20 |

### Sample 218

- **Input**: `UFA44 เลือกเล่น Joker ที่จังหวะเล่นคุ้นมือ เกมไม่ซับซ้อน ฟีเจอร์อ่านง่าย เหมาะกับคนที่ชอบความคลาสสิก`
- **Prefix keyword**: `Some("ufa44")`
- **Sub-tokens**: `["ufa", "44"]`
- **Primary intent**: `Navigational` (confidence: 59%)

| Keyword | Score |
|---------|-------|
| ufa44 | 1.00 |
| ufa | 0.80 |
| 44 | 0.80 |
| เล่น | 0.60 |
| joker | 0.20 |

### Sample 219

- **Input**: `UFA696 โฟกัสที่อัตราชนะสูงด้วยการตั้งเป้าหมายเป็นรอบ แบ่งงบให้ชัด รู้จังหวะพัก และเลือกเกมที่เหมาะกั`
- **Prefix keyword**: `Some("ufa696")`
- **Sub-tokens**: `["ufa", "696"]`
- **Primary intent**: `Navigational` (confidence: 60%)

| Keyword | Score |
|---------|-------|
| ufa696 | 1.00 |
| ufa | 0.80 |
| 696 | 0.80 |
| เล่น | 0.40 |

### Sample 220

- **Input**: `UFA85AUTO LOGIN สัมผัส Live Casino แบบถ่ายทอดสดจริง เลือกโต๊ะง่าย ระบบลื่นบนมือถือ ทำให้เล่นได้ทุกที`
- **Prefix keyword**: `Some("ufa85auto")`
- **Sub-tokens**: `["ufa", "85", "auto"]`
- **Primary intent**: `Navigational` (confidence: 53%)

| Keyword | Score |
|---------|-------|
| ufa85auto | 1.00 |
| ufa | 0.80 |
| 85 | 0.80 |
| auto | 0.80 |
| เล่น | 0.20 |
| casino | 0.20 |

### Sample 221

- **Input**: `UFABET ICON รวม PG Slot ที่มีธีมหลากหลายและฟีเจอร์ฟรีสปินน่าลุ้น เล่นลื่นบนมือถือ เหมาะกับคนที่อยากไ`
- **Prefix keyword**: `Some("ufabet")`
- **Sub-tokens**: `["uf", "abet"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ufabet | 1.00 |
| uf | 0.80 |
| abet | 0.80 |
| เล่น | 0.40 |
| ufa | 0.20 |
| pg slot | 0.20 |
| bet | 0.20 |
| slot | 0.20 |
| ฟรี | 0.20 |

### Sample 222

- **Input**: `UFABETS RTP API ช่วยให้คุณเห็นข้อมูลประกอบการเลือกเกมได้สะดวกขึ้น ลดการเดาสุ่ม และช่วยให้วางแผนการเล`
- **Prefix keyword**: `Some("ufabets")`
- **Sub-tokens**: `["uf", "abets"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ufabets | 1.00 |
| uf | 0.80 |
| abets | 0.80 |
| เล่น | 0.80 |
| bet | 0.40 |
| joker | 0.20 |
| กติกา | 0.20 |
| ufa | 0.20 |

### Sample 223

- **Input**: `UFABETWIN LOGIN รวม JILI Slot ที่ตอบสนองเร็ว เล่นต่อเนื่องได้ดี ฟีเจอร์ช่วยให้ลุ้นสนุก เหมาะกับคนที่`
- **Prefix keyword**: `Some("ufabetwin")`
- **Sub-tokens**: `["uf", "abet", "win"]`
- **Primary intent**: `Navigational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ufabetwin | 1.00 |
| uf | 0.80 |
| abet | 0.80 |
| win | 0.80 |
| เล่น | 0.40 |
| bet | 0.20 |
| slot | 0.20 |
| jili | 0.20 |
| ufa | 0.20 |

### Sample 224

- **Input**: `UFAPERFECT รวม JILI Slot ที่เล่นลื่น หมุนเร็ว ฟีเจอร์เด่นทำให้ลุ้นได้บ่อย เหมาะกับคนชอบเกมสปีดไวและอ`
- **Prefix keyword**: `Some("ufaperfect")`
- **Sub-tokens**: `["uf", "aper", "fect"]`
- **Primary intent**: `Navigational` (confidence: 59%)

| Keyword | Score |
|---------|-------|
| ufaperfect | 1.00 |
| uf | 0.80 |
| aper | 0.80 |
| fect | 0.80 |
| เล่น | 0.40 |
| jili | 0.20 |
| slot | 0.20 |
| ufa | 0.20 |

### Sample 225

- **Input**: `UFAZEED รวมเกม SpadeGaming ที่ออกแบบมาเล่นง่าย ฟีเจอร์พอดีและภาพสวย ช่วยให้เล่นเพลินบนมือถือ เหมาะกั`
- **Prefix keyword**: `Some("ufazeed")`
- **Sub-tokens**: `["uf", "aze", "ed"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| ufazeed | 1.00 |
| uf | 0.80 |
| aze | 0.80 |
| ed | 0.80 |
| เล่น | 0.60 |
| spade | 0.20 |
| ufa | 0.20 |

### Sample 226

- **Input**: `ufa แทงบอล ทุกครั้งที่คุณเล่น จะได้รับแต้มพิเศษ ใช้แลกรางวัล หรือเข้าสู่เกมพรีเมียมได้แบบไม่ต้องจ่าย`
- **Prefix keyword**: `Some("ufa")`
- **Sub-tokens**: `["ufa"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| ufa | 1.00 |
| เล่น | 0.40 |
| ลงทะเบียน | 0.20 |
| รางวัล | 0.20 |
| พิเศษ | 0.20 |

### Sample 227

- **Input**: `ufa สล็อต นอกจากนี้ ระบบสมาชิกสุดพิเศษของเรา จะทำให้คุณเจอกับสิทธิประโยชน์มากมาย ไม่ว่าจะเป็นของขวัญ`
- **Prefix keyword**: `Some("ufa")`
- **Sub-tokens**: `["ufa"]`
- **Primary intent**: `Navigational` (confidence: 55%)

| Keyword | Score |
|---------|-------|
| ufa | 1.00 |
| พิเศษ | 0.20 |
| สมัคร | 0.20 |
| โปรโมชั่น | 0.20 |
| สล็อต | 0.20 |
| สมาชิก | 0.20 |
| ของขวัญ | 0.20 |

### Sample 228

- **Input**: `UPSX789 สำหรับคนที่เล่น สล็อตออนไลน์ บ่อย ๆ แนะนำให้เลือกเกมตามงบและเวลา ตั้งเป้าเป็นรอบ ๆ และทำให้ก`
- **Prefix keyword**: `Some("upsx789")`
- **Sub-tokens**: `["ups", "789"]`
- **Primary intent**: `Informational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| upsx789 | 1.00 |
| ups | 0.80 |
| 789 | 0.80 |
| เล่น | 0.60 |
| สล็อต | 0.20 |
| แนะนำ | 0.20 |

### Sample 229

- **Input**: `USA999 รวม PG Slot ที่ภาพสวยและลูกเล่นแน่น ทำให้ลุ้นฟรีสปินได้สนุก เล่นเพลินบนมือถือ และเหมาะกับคนที`
- **Prefix keyword**: `Some("usa999")`
- **Sub-tokens**: `["usa", "999"]`
- **Primary intent**: `Navigational` (confidence: 20%)

| Keyword | Score |
|---------|-------|
| usa999 | 1.00 |
| usa | 0.80 |
| 999 | 0.80 |
| เล่น | 0.60 |
| รายละเอียด | 0.20 |
| slot | 0.20 |
| ฟรี | 0.20 |
| pg slot | 0.20 |

### Sample 230

- **Input**: `VIDEOS; 360° VIEW; IMAGES. หวยเวียดนาม>Visit the PlayStation Store. Visit the PlayStation Store. Pla`
- **Prefix keyword**: `Some("videos")`
- **Sub-tokens**: `["videos"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| videos | 1.00 |

### Sample 231

- **Input**: `W1688 ด้วย RTP API คุณสามารถดูข้อมูลเกมเพื่อประกอบการเลือกเล่นได้ง่ายขึ้น ช่วยให้ตัดสินใจแบบมีเหตุผล`
- **Prefix keyword**: `Some("w1688")`
- **Sub-tokens**: `["168"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| w1688 | 1.00 |
| 168 | 0.80 |
| เล่น | 0.40 |

### Sample 232

- **Input**: `Welcome to หวยเวียดนามย้อนหลัง เครดิตฟรี200 pg— The Realm of Intense Gaming!  . หวยเวียดนามย้อนหลัง `
- **Prefix keyword**: `Some("welcome")`
- **Sub-tokens**: `["welcome"]`
- **Primary intent**: `Commercial` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| welcome | 1.00 |
| เครดิตฟรี | 0.20 |
| slot | 0.20 |
| เล่น | 0.20 |
| ฟรี | 0.20 |
| โบนัส | 0.20 |

### Sample 233

- **Input**: `www.777.com ลิ้งเข้าระบบ การสมัครง่าย,เข้าถึงแพลตฟอร์มเกมได้ทันที,รวมเกมจากผู้ให้บริการชื่อดังทั้งหม`
- **Prefix keyword**: `Some("www")`
- **Sub-tokens**: `["www"]`
- **Primary intent**: `Navigational` (confidence: 54%)

| Keyword | Score |
|---------|-------|
| www | 1.00 |
| สมัคร | 0.40 |
| slot | 0.40 |
| 888 | 0.20 |
| ufa | 0.20 |
| เข้าระบบ | 0.20 |
| bet | 0.20 |

### Sample 234

- **Input**: `WWW.GOAL เลือก JILI Slot ที่หมุนไวและไม่หน่วง ฟีเจอร์ทำให้ลุ้นได้บ่อย เหมาะกับคนที่อยากได้เกมสนุกแบบ`
- **Prefix keyword**: `Some("www")`
- **Sub-tokens**: `["www"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| www | 1.00 |
| slot | 0.20 |
| jili | 0.20 |
| goal | 0.20 |

### Sample 235

- **Input**: `XPAY88 แนว Lucky Win ทำให้การเล่นสนุกขึ้นด้วยโปรที่เข้าถึงง่าย เงื่อนไขไม่ซับซ้อน เหมาะกับคนที่อยากไ`
- **Prefix keyword**: `Some("xpay88")`
- **Sub-tokens**: `["pay", "88"]`
- **Primary intent**: `Informational` (confidence: 28%)

| Keyword | Score |
|---------|-------|
| xpay88 | 1.00 |
| pay | 0.80 |
| 88 | 0.80 |
| เงื่อนไข | 0.40 |
| เล่น | 0.40 |
| รายละเอียด | 0.20 |

### Sample 236

- **Input**: `XWIN-888 Baccarat เล่นไม่ซับซ้อน เหมาะกับคนเริ่มใหม่และคนมีเวลาน้อย แนะนำการเล่นเป็นรอบ คุมงบ และหยุ`
- **Prefix keyword**: `Some("xwin")`
- **Sub-tokens**: `["win"]`
- **Primary intent**: `Informational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| xwin | 1.00 |
| win | 0.80 |
| เล่น | 0.80 |
| แนะนำ | 0.20 |
| 888 | 0.20 |

### Sample 237

- **Input**: `YUKITO สัมผัส Live Casino แบบถ่ายทอดสดจริง เลือกโต๊ะง่าย ระบบลื่นบนมือถือ ทำให้เล่นได้ทุกที่ เหมาะกั`
- **Prefix keyword**: `Some("yukito")`
- **Sub-tokens**: `["uk", "ito"]`
- **Primary intent**: `Informational` (confidence: 7%)

| Keyword | Score |
|---------|-------|
| yukito | 1.00 |
| uk | 0.80 |
| ito | 0.80 |
| เล่น | 0.20 |
| casino | 0.20 |

### Sample 238

- **Input**: `Z888 สัมผัส Live Casino กับดีลเลอร์จริง ภาพสดเสียงชัด ระบบลื่นบนมือถือ เหมาะกับคนที่ชอบความเรียลและอ`
- **Prefix keyword**: `Some("z888")`
- **Sub-tokens**: `["888"]`
- **Primary intent**: `Navigational` (confidence: 91%)

| Keyword | Score |
|---------|-------|
| z888 | 1.00 |
| 888 | 0.80 |
| casino | 0.20 |
| เล่น | 0.20 |

### Sample 239

- **Input**: `ZUKA168 สัมผัส Live Casino กับดีลเลอร์จริง ภาพสดเสียงชัด ระบบลื่นบนมือถือ เหมาะกับคนที่ชอบความเรียลแ`
- **Prefix keyword**: `Some("zuka168")`
- **Sub-tokens**: `["uka", "168"]`
- **Primary intent**: `Informational` (confidence: 7%)

| Keyword | Score |
|---------|-------|
| zuka168 | 1.00 |
| uka | 0.80 |
| 168 | 0.80 |
| เล่น | 0.20 |
| casino | 0.20 |

### Sample 240

- **Input**: `เกมสล็อตทดลองเล่น ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นของขวัญ `
- **Prefix keyword**: `Some("เกมสล\u{e47}อตทดลองเล\u{e48}น")`
- **Sub-tokens**: `["เกมสล\u{e47}อต", "ทดลอง", "เล\u{e48}น"]`
- **Primary intent**: `Informational` (confidence: 86%)

| Keyword | Score |
|---------|-------|
| เกมสล็อตทดลองเล่น | 1.00 |
| เกมสล็อต | 0.80 |
| ทดลอง | 0.80 |
| เล่น | 0.80 |
| สมาชิก | 0.20 |
| ของขวัญ | 0.20 |
| สล็อต | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 241

- **Input**: `คณะเศรษฐศาสตร์ มข. ขับเคลื่อนหลักสูตรเศรษฐศาสตร์สู่อนาคต จัดเวทีเสวนาระดมความคิดพลิกโฉมหลักสูตรด้วยพ`
- **Prefix keyword**: `Some("คณะเศรษฐศาสตร\u{e4c}")`
- **Sub-tokens**: `["ค", "ณะ", "เศ", "ร", "ษ", "ฐ", "ศาสตร\u{e4c}"]`
- **Primary intent**: `Commercial` (confidence: 97%)

| Keyword | Score |
|---------|-------|
| คณะเศรษฐศาสตร์ | 1.00 |
| ค | 0.80 |
| ณะ | 0.80 |
| เศ | 0.80 |
| ร | 0.80 |
| ษ | 0.80 |
| ฐ | 0.80 |
| ศาสตร์ | 0.80 |

### Sample 242

- **Input**: `ค้นหาโอกาสในการรวยอย่างรวดเร็ว สโบเบ็ต888 ประสบการณ์ผู้ใช้ระดับพรีเมียมและการบริการลูกค้าที่เป็นเลิศ`
- **Prefix keyword**: `Some("ค\u{e49}นหาโอกาสในการรวยอย\u{e48}างรวดเร\u{e47}ว")`
- **Sub-tokens**: `["ค\u{e49}น", "หา", "โ", "อก", "าส", "ในการ", "ร", "วย", "อย\u{e48}าง", "รว", "ด", "เร\u{e47}ว"]`
- **Primary intent**: `Commercial` (confidence: 67%)

| Keyword | Score |
|---------|-------|
| ค้นหาโอกาสในการรวยอย่างรวดเร็ว | 1.00 |
| ค้น | 0.80 |
| หา | 0.80 |
| โ | 0.80 |
| อก | 0.80 |
| าส | 0.80 |
| ในการ | 0.80 |
| ร | 0.80 |
| วย | 0.80 |
| อย่าง | 0.80 |
| รว | 0.80 |
| ด | 0.80 |
| เร็ว | 0.80 |
| 888 | 0.20 |
| โบนัส | 0.20 |

### Sample 243

- **Input**: `เครดิตฟรี188ไม่ต้องฝากไม่ต้องแชร์ สนใจที่จะสัมผัสประสบการณ์ใหม่ล่าสุดกับเราไหม? ที่นี่มีเกมคาสิโนและ`
- **Prefix keyword**: `Some("เครด\u{e34}ตฟร\u{e35}")`
- **Sub-tokens**: `["เครด\u{e34}ตฟร\u{e35}"]`
- **Primary intent**: `Commercial` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| เครดิตฟรี | 1.00 |
| เล่น | 0.20 |
| ไม่ต้องฝาก | 0.20 |
| ฝาก | 0.20 |
| ฟรี | 0.20 |

### Sample 244

- **Input**: `เครดิตฟรีไม่ต้องฝาก ทุกครั้งที่คุณเล่น จะได้รับแต้มพิเศษ ใช้แลกรางวัล หรือเข้าสู่เกมพรีเมียมได้แบบไม`
- **Prefix keyword**: `Some("เครด\u{e34}ตฟร\u{e35}ไม\u{e48}ต\u{e49}องฝาก")`
- **Sub-tokens**: `["เครด\u{e34}ตฟร\u{e35}", "ไม\u{e48}ต\u{e49}องฝาก"]`
- **Primary intent**: `Commercial` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| เครดิตฟรีไม่ต้องฝาก | 1.00 |
| เครดิตฟรี | 0.80 |
| ไม่ต้องฝาก | 0.80 |
| ฟรี | 0.20 |
| ฝาก | 0.20 |
| เล่น | 0.20 |
| พิเศษ | 0.20 |
| รางวัล | 0.20 |

### Sample 245

- **Input**: `เครดิตฟรีไม่ฝากไม่แชร์ เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแคลงทะเบียน คุณจะได้รับประสบการณ์สุดอร`
- **Prefix keyword**: `Some("เครด\u{e34}ตฟร\u{e35}ไม\u{e48}ฝากไม\u{e48}แชร\u{e4c}")`
- **Sub-tokens**: `["เครด\u{e34}ตฟร\u{e35}", "ไม\u{e48}", "ฝาก", "ไม\u{e48}", "แชร\u{e4c}"]`
- **Primary intent**: `Commercial` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| เครดิตฟรีไม่ฝากไม่แชร์ | 1.00 |
| เครดิตฟรี | 0.80 |
| ไม่ | 0.80 |
| ฝาก | 0.80 |
| แชร์ | 0.80 |
| ลงทะเบียน | 0.20 |
| สล็อต | 0.20 |
| ฟรี | 0.20 |
| รูเล็ต | 0.20 |

### Sample 246

- **Input**: `โค้ดเฮงเฮง888 นอกจากนี้ ระบบสมาชิกออกแบบเพื่อคุณอย่างใกล้ชิด ไม่ว่าจะเป็นของขวัญ, แต้มสะสม , หรือโปร`
- **Prefix keyword**: `Some("โค\u{e49}ดเฮงเฮง")`
- **Sub-tokens**: `["โ", "ค\u{e49}", "ด", "ง", "ง"]`
- **Primary intent**: `Commercial` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| โค้ดเฮงเฮง | 1.00 |
| โ | 0.80 |
| ค้ | 0.80 |
| ด | 0.80 |
| ง | 0.80 |
| เล่น | 0.40 |
| สมาชิก | 0.20 |
| ของขวัญ | 0.20 |
| 888 | 0.20 |

### Sample 247

- **Input**: `โครงการขุดเจาะบ่อบาดาลหมู่ที่ 3 ต.ท่ามะเดื่อ อ.บางแก้ว จ.พัทลุง. SLOT ONLINE · ดาวน์โหลดเอกสารเพิ่มเ`
- **Prefix keyword**: `Some("โครงการข\u{e38}ดเจาะบ\u{e48}อบาดาลหม\u{e39}\u{e48}ท\u{e35}\u{e48}")`
- **Sub-tokens**: `["โ", "ครงการ", "ข", "\u{e38}ด", "เจ", "าะ", "บ", "\u{e48}อ", "บ", "าด", "าล", "หม", "\u{e39}\u{e48}", "ท\u{e35}\u{e48}"]`
- **Primary intent**: `Transactional` (confidence: 55%)

| Keyword | Score |
|---------|-------|
| โครงการขุดเจาะบ่อบาดาลหมู่ที่ | 1.00 |
| โ | 0.80 |
| ครงการ | 0.80 |
| ข | 0.80 |
| ุด | 0.80 |
| เจ | 0.80 |
| าะ | 0.80 |
| บ | 0.80 |
| ่อ | 0.80 |
| าด | 0.80 |
| าล | 0.80 |
| หม | 0.80 |
| ู่ | 0.80 |
| ที่ | 0.80 |
| slot | 0.20 |

### Sample 248

- **Input**: `โครตปัง888สล็อต คัดสล็อตแตกง่าย RTP สูง ฟรีสปินมาไว ภารกิจรายวันพร้อมโบนัสต่อเนื่อง ฝากไวถอนเร็วระบบ`
- **Prefix keyword**: `Some("โครตป\u{e31}ง")`
- **Sub-tokens**: `["โ", "คร", "ต", "ป", "\u{e31}ง"]`
- **Primary intent**: `Commercial` (confidence: 85%)

| Keyword | Score |
|---------|-------|
| โครตปัง | 1.00 |
| โ | 0.80 |
| คร | 0.80 |
| ต | 0.80 |
| ป | 0.80 |
| ัง | 0.80 |
| สล็อต | 0.40 |
| เล่น | 0.20 |
| ฝาก | 0.20 |
| โบนัส | 0.20 |
| 888 | 0.20 |
| ถอน | 0.20 |
| ฟรี | 0.20 |

### Sample 249

- **Input**: `แจกเครดิตฟรี+ไม่ต้องฝาก+ไม่ต้องแชร์+ล่าสุด ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็`
- **Prefix keyword**: `Some("แจกเครด\u{e34}ตฟร\u{e35}")`
- **Sub-tokens**: `["แจก", "เครด\u{e34}ตฟร\u{e35}"]`
- **Primary intent**: `Commercial` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| แจกเครดิตฟรี | 1.00 |
| แจก | 0.80 |
| เครดิตฟรี | 0.80 |
| ไม่ต้องฝาก | 0.20 |
| ฝาก | 0.20 |
| โปรโมชั่น | 0.20 |
| ของขวัญ | 0.20 |
| สมาชิก | 0.20 |
| ฟรี | 0.20 |

### Sample 250

- **Input**: `เช็คผล บอล 888 พร้อม ราคา เมื่อ คืน ที่เว็บ SportMaster! อัปเดตทุกแมตช์ ทันใจทุกลีก พร้อมโปรโมชั่นพิ`
- **Prefix keyword**: `Some("เช\u{e47}คผล")`
- **Sub-tokens**: `["เช", "\u{e47}ค", "ผล"]`
- **Primary intent**: `Commercial` (confidence: 21%)

| Keyword | Score |
|---------|-------|
| เช็คผล | 1.00 |
| เช | 0.80 |
| ็ค | 0.80 |
| ผล | 0.80 |
| โปรโมชั่น | 0.40 |
| พิเศษ | 0.40 |
| 888 | 0.40 |
| โบนัส | 0.40 |
| สล็อต | 0.20 |
| สมัคร | 0.20 |
| สมาชิก | 0.20 |
| เล่น | 0.20 |

### Sample 251

- **Input**: `ดูบอลสด ufa ผู้ชื่นชอบเกมแนวหมุนวงล้อหรือสายกีฬา ที่นี่มีให้ครบ พร้อมสิทธิ์ทดลองและของขวัญหลังลงทะเบ`
- **Prefix keyword**: `Some("ด\u{e39}บอลสด")`
- **Sub-tokens**: `["ด\u{e39}", "บอลสด"]`
- **Primary intent**: `Informational` (confidence: 11%)

| Keyword | Score |
|---------|-------|
| ดูบอลสด | 1.00 |
| ดู | 0.80 |
| บอลสด | 0.80 |
| ทดลอง | 0.20 |
| ของขวัญ | 0.20 |
| ufa | 0.20 |
| ลงทะเบียน | 0.20 |
| เล่น | 0.20 |

### Sample 252

- **Input**: `ดูหนังออนไลน์ฟรีหนัง นอกจากนี้ ระบบสมาชิกของเรายังมีการให้ของขวัญ โปรโมชั่นรายสัปดาห์ และแม้แต่วิธีส`
- **Prefix keyword**: `Some("ด\u{e39}หน\u{e31}งออนไลน\u{e4c}ฟร\u{e35}หน\u{e31}ง")`
- **Sub-tokens**: `["ด\u{e39}", "หน\u{e31}ง", "ออนไลน\u{e4c}", "ฟร\u{e35}", "หน\u{e31}ง"]`
- **Primary intent**: `Commercial` (confidence: 61%)

| Keyword | Score |
|---------|-------|
| ดูหนังออนไลน์ฟรีหนัง | 1.00 |
| ดู | 0.80 |
| หนัง | 0.80 |
| ออนไลน์ | 0.80 |
| ฟรี | 0.80 |
| สมาชิก | 0.20 |
| วิธี | 0.20 |
| ของขวัญ | 0.20 |
| รางวัล | 0.20 |
| สล็อต | 0.20 |
| พิเศษ | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 253

- **Input**: `ดูหนังออนไลน์ฟรีหนัง ระบบใหม่ล่าสุด มั่นใจด้วย API ลิขสิทธิ์แท้ พร้อมกิจกรรมรายวันที่อัปเดตอย่างต่อเ`
- **Prefix keyword**: `Some("ด\u{e39}หน\u{e31}งออนไลน\u{e4c}ฟร\u{e35}หน\u{e31}ง")`
- **Sub-tokens**: `["ด\u{e39}", "หน\u{e31}ง", "ออนไลน\u{e4c}", "ฟร\u{e35}", "หน\u{e31}ง"]`
- **Primary intent**: `Commercial` (confidence: 63%)

| Keyword | Score |
|---------|-------|
| ดูหนังออนไลน์ฟรีหนัง | 1.00 |
| ดู | 0.80 |
| หนัง | 0.80 |
| ออนไลน์ | 0.80 |
| ฟรี | 0.80 |
| เล่น | 0.20 |
| รางวัล | 0.20 |
| พิเศษ | 0.20 |

### Sample 254

- **Input**: `ดู หนัง ออนไลน์ยิ่คะแนนสะสมไม่อั้น แลกของจริง อัปเดตใหญ่ 2026 รับคะแนนฟรีทันที ทดลองเล่นฟรี 2026 ไม่`
- **Prefix keyword**: `Some("ด\u{e39}")`
- **Sub-tokens**: `["ด\u{e39}"]`
- **Primary intent**: `Commercial` (confidence: 46%)

| Keyword | Score |
|---------|-------|
| ดู | 1.00 |
| ฟรี | 0.40 |
| รางวัล | 0.20 |
| ทดลอง | 0.20 |
| ถอน | 0.20 |
| เล่น | 0.20 |
| ขั้นต่ำ | 0.20 |
| ฝาก | 0.20 |

### Sample 255

- **Input**: `ดู​หนังออนไลน์ ระบบ API ลิขสิทธิ์แท้ ทดลองฟรี 2026 ทดลองฟรี ไม่ต้องลงทุน เริ่มเลย! แชร์คะแนน เพิ่มโอ`
- **Prefix keyword**: `Some("ด\u{e39}")`
- **Sub-tokens**: `["ด\u{e39}"]`
- **Primary intent**: `Commercial` (confidence: 64%)

| Keyword | Score |
|---------|-------|
| ดู | 1.00 |
| ฟรี | 0.80 |
| ทดลอง | 0.60 |
| เล่น | 0.20 |
| รางวัล | 0.20 |

### Sample 256

- **Input**: `เด็กไทยชั้นป. 1 มีระดับสติปัญญา(IQ)เฉลี่ยเท่ากับ 102.8 เกณฑ์ปกติ (90-110) ความฉลาดทางอารมณ์(EQ) อยู่`
- **Prefix keyword**: `Some("เด\u{e47}กไทยช\u{e31}\u{e49}นป")`
- **Sub-tokens**: `["เด\u{e47}ก", "ไทย", "ช", "\u{e31}\u{e49}น", "ป"]`
- **Primary intent**: `Transactional` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| เด็กไทยชั้นป | 1.00 |
| เด็ก | 0.80 |
| ไทย | 0.80 |
| ช | 0.80 |
| ั้น | 0.80 |
| ป | 0.80 |

### Sample 257

- **Input**: `ตรวจหวยผลสลากกินแบ่งรัฐบาลงวด 2563ยิ่ร่วมสนุกกับเพื่อน แชร์แต้ม ฟรี 100% คลิกเลย! ระบบออโต้แท้ ได้เง`
- **Prefix keyword**: `Some("ตรวจหวยผลสลากก\u{e34}นแบ\u{e48}งร\u{e31}ฐบาลงวด")`
- **Sub-tokens**: `["ตรวจ", "หวย", "ผล", "ส", "ลาก", "ก\u{e34}นแบ\u{e48}ง", "ร\u{e31}ฐบาล", "ง", "วด"]`
- **Primary intent**: `Commercial` (confidence: 61%)

| Keyword | Score |
|---------|-------|
| ตรวจหวยผลสลากกินแบ่งรัฐบาลงวด | 1.00 |
| ตรวจ | 0.80 |
| หวย | 0.80 |
| ผล | 0.80 |
| ส | 0.80 |
| ลาก | 0.80 |
| กินแบ่ง | 0.80 |
| รัฐบาล | 0.80 |
| ง | 0.80 |
| วด | 0.80 |
| ฟรี | 0.40 |
| ทดลอง | 0.20 |
| สล็อต | 0.20 |
| เล่น | 0.20 |

### Sample 258

- **Input**: `ตรวจหวยสลากกินแบ่งรัฐบาล สำหรับคุณที่ชอบแนวหมุนวงล้อ หรือสายกีฬา ที่นี่มีให้ครบ พร้อมสิทธิ์ทดลองและข`
- **Prefix keyword**: `Some("ตรวจหวยสลากก\u{e34}นแบ\u{e48}งร\u{e31}ฐบาล")`
- **Sub-tokens**: `["ตรวจ", "หวย", "ส", "ลาก", "ก\u{e34}นแบ\u{e48}ง", "ร\u{e31}ฐบาล"]`
- **Primary intent**: `Commercial` (confidence: 41%)

| Keyword | Score |
|---------|-------|
| ตรวจหวยสลากกินแบ่งรัฐบาล | 1.00 |
| ตรวจ | 0.80 |
| หวย | 0.80 |
| ส | 0.80 |
| ลาก | 0.80 |
| กินแบ่ง | 0.80 |
| รัฐบาล | 0.80 |
| ของขวัญ | 0.20 |
| ทดลอง | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 259

- **Input**: `ตลาดลูกหนัง สปอร์ตพูล สปอร์ตแมน วิเคราะห์บอล ทีเด็ดบอล ที่นี่มีทั้งเกมแนวหมุนวงล้อ เกมกระดาน และสายก`
- **Prefix keyword**: `Some("ตลาดล\u{e39}กหน\u{e31}ง")`
- **Sub-tokens**: `["ตลาด", "ล\u{e39}ก", "หน\u{e31}ง"]`
- **Primary intent**: `Informational` (confidence: 5%)

| Keyword | Score |
|---------|-------|
| ตลาดลูกหนัง | 1.00 |
| ตลาด | 0.80 |
| ลูก | 0.80 |
| หนัง | 0.80 |
| รางวัล | 0.20 |
| เล่น | 0.20 |

### Sample 260

- **Input**: `ตารางแข่งบอลพรุ่งนี้ ฝากถอนไว + คะแนนพิเศษทุกวัน แชร์คะแนน ร่วมสนุก ไม่มีค่าใช้จ่าย สนุกไม่อั้น คะแน`
- **Prefix keyword**: `Some("ตารางแข\u{e48}งบอลพร\u{e38}\u{e48}งน\u{e35}\u{e49}")`
- **Sub-tokens**: `["ต", "าราง", "แข", "\u{e48}ง", "บอล", "พ", "ร\u{e38}\u{e48}งน\u{e35}\u{e49}"]`
- **Primary intent**: `Commercial` (confidence: 44%)

| Keyword | Score |
|---------|-------|
| ตารางแข่งบอลพรุ่งนี้ | 1.00 |
| ต | 0.80 |
| าราง | 0.80 |
| แข | 0.80 |
| ่ง | 0.80 |
| บอล | 0.80 |
| พ | 0.80 |
| รุ่งนี้ | 0.80 |
| ฟรี | 0.40 |
| ถอน | 0.20 |
| ทดลอง | 0.20 |
| ลงทะเบียน | 0.20 |
| พิเศษ | 0.20 |
| ฝาก | 0.20 |
| เล่น | 0.20 |

### Sample 261

- **Input**: `ตำรวจลอตเตอรี่ ฝาก 20 รับ 100,แพลตฟอร์มตรงอันดับ 1 ล่าสุด,รวมเกมจากผู้ให้บริการชื่อดังทั้งหมดpgหลัก、`
- **Prefix keyword**: `Some("ตำรวจลอตเตอร\u{e35}\u{e48}")`
- **Sub-tokens**: `["ตำรวจ", "ล", "อตเตอร\u{e35}\u{e48}"]`
- **Primary intent**: `Informational` (confidence: 63%)

| Keyword | Score |
|---------|-------|
| ตำรวจลอตเตอรี่ | 1.00 |
| ตำรวจ | 0.80 |
| ล | 0.80 |
| อตเตอรี่ | 0.80 |
| slot | 0.20 |
| ฝาก | 0.20 |

### Sample 262

- **Input**: `ตำรวจหวย ⚙️ ไม่ผ่านตัวแทน,ฝาก-ถอนตลอด 24 ชั่วโมง,รวมเกมจากผู้ให้บริการชื่อดังทั้งหมดjdbaa slot、z9 sl`
- **Prefix keyword**: `Some("ตำรวจหวย")`
- **Sub-tokens**: `["ตำรวจ", "หวย"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| ตำรวจหวย | 1.00 |
| ตำรวจ | 0.80 |
| หวย | 0.80 |
| slot | 1.00 |
| ฝาก | 0.20 |
| ถอน | 0.20 |

### Sample 263

- **Input**: `ถ่ายทอดบอลพรีเมียร์ลีกอังกฤษ เรามีฐานการเงิน – เว็บไซต์ทางการ แพลตฟอร์มเดิมพัน & ข่าวสารอันดับ 1 ในไ`
- **Prefix keyword**: `Some("ถ\u{e48}ายทอดบอลพร\u{e35}เม\u{e35}ยร\u{e4c}ล\u{e35}กอ\u{e31}งกฤษ")`
- **Sub-tokens**: `["ถ", "\u{e48}ายทอด", "บอล", "พร\u{e35}เม\u{e35}ยร\u{e4c}ล\u{e35}ก", "อ\u{e31}งกฤษ"]`
- **Primary intent**: `Transactional` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| ถ่ายทอดบอลพรีเมียร์ลีกอังกฤษ | 1.00 |
| ถ | 0.80 |
| ่ายทอด | 0.80 |
| บอล | 0.80 |
| พรีเมียร์ลีก | 0.80 |
| อังกฤษ | 0.80 |

### Sample 264

- **Input**: `ทางเข้า777 โปรโมชั่นล่าสุด,ฝาก 20 รับ 100,แพลตฟอร์มตรงอันดับ 1 ล่าสุด,รวมเกมจากผู้ให้บริการชื่อดังทั`
- **Prefix keyword**: `Some("ทางเข\u{e49}า")`
- **Sub-tokens**: `["ทาง", "เข\u{e49}า"]`
- **Primary intent**: `Transactional` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ทางเข้า | 1.00 |
| ทาง | 0.80 |
| เข้า | 0.80 |
| ฝาก | 0.20 |
| slot | 0.20 |
| โปรโมชั่น | 0.20 |
| ufa | 0.20 |

### Sample 265

- **Input**: `ทางเข้า777 ไม่จำเป็นต้องฝากเงิน แค่เข้าระบบคุณก็สามารถทดลองเล่น สล็อต PG รูเล็ต และเกมกีฬาหลากหลายแน`
- **Prefix keyword**: `Some("ทางเข\u{e49}า")`
- **Sub-tokens**: `["ทาง", "เข\u{e49}า"]`
- **Primary intent**: `Transactional` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ทางเข้า | 1.00 |
| ทาง | 0.80 |
| เข้า | 0.80 |
| ฝาก | 0.20 |
| เข้าระบบ | 0.20 |
| รูเล็ต | 0.20 |
| สล็อต | 0.20 |
| ทดลอง | 0.20 |
| ฝากเงิน | 0.20 |
| สมาชิก | 0.20 |

### Sample 266

- **Input**: `ทางเข้ายูฟ่า777 เริ่มต้นความสนุกแบบง่ายๆ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลายทั้งสล็อต, รูเล็`
- **Prefix keyword**: `Some("ทางเข\u{e49}าย\u{e39}ฟ\u{e48}า")`
- **Sub-tokens**: `["ทาง", "เข\u{e49}", "าย", "\u{e39}", "ฟ\u{e48}า"]`
- **Primary intent**: `Transactional` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ทางเข้ายูฟ่า | 1.00 |
| ทาง | 0.80 |
| เข้ | 0.80 |
| าย | 0.80 |
| ู | 0.80 |
| ฟ่า | 0.80 |
| ทางเข้า | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝาก | 0.20 |
| ฝากเงิน | 0.20 |
| เข้าระบบ | 0.20 |
| รูเล็ต | 0.20 |
| สล็อต | 0.20 |

### Sample 267

- **Input**: `ทาง เข้า สล็อต 777 เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้`
- **Prefix keyword**: `Some("ทาง")`
- **Sub-tokens**: `["ทาง"]`
- **Primary intent**: `Transactional` (confidence: 91%)

| Keyword | Score |
|---------|-------|
| ทาง | 1.00 |
| สล็อต | 0.40 |
| ฝากเงิน | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝาก | 0.20 |
| รูเล็ต | 0.20 |

### Sample 268

- **Input**: `แทงบอล ufa พร้อมเปิดประสบการณ์ใหม่ล่าสุด! มั่นใจด้วย API ลิขสิทธิ์แท้ พร้อมกิจกรรมรายวันที่อัปเดตอย่`
- **Prefix keyword**: `Some("แทงบอล")`
- **Sub-tokens**: `["แทงบอล"]`
- **Primary intent**: `Transactional` (confidence: 73%)

| Keyword | Score |
|---------|-------|
| แทงบอล | 1.00 |
| ถอน | 0.40 |
| ฝาก | 0.40 |
| เล่น | 0.20 |
| สล็อต | 0.20 |
| เข้าสู่ระบบ | 0.20 |
| ขั้นต่ำ | 0.20 |
| ufa | 0.20 |
| ทางเข้า | 0.20 |

### Sample 269

- **Input**: `ไทย สล็อต 888ยิ่2026 ระบบอัตโนมัติสุดล้ำ ทดลองฟรี แชร์คะแนนกับเพื่อน รวยแต้มกว่าเดิม ร่วมสนุกกับระบบ`
- **Prefix keyword**: `Some("ไทย")`
- **Sub-tokens**: `["ไทย"]`
- **Primary intent**: `Commercial` (confidence: 65%)

| Keyword | Score |
|---------|-------|
| ไทย | 0.71 |
| ฟรี | 1.00 |
| ทดลอง | 0.86 |
| 888 | 0.29 |
| สล็อต | 0.29 |
| โบนัส | 0.14 |

### Sample 270

- **Input**: `ไทย สล็อต 888 เวทีการประชุมวิชาการระดับชาติ มหาวิทยาลัยทักษิณ ปี 2568 ครอบคลุมหลายสาขาวิชา สนับสนุนก`
- **Prefix keyword**: `Some("ไทย")`
- **Sub-tokens**: `["ไทย"]`
- **Primary intent**: `Navigational` (confidence: 44%)

| Keyword | Score |
|---------|-------|
| ไทย | 1.00 |
| สล็อต | 0.40 |
| 888 | 0.40 |

### Sample 271

- **Input**: `นางสาวชัญญณัฐ พูลอ่อน ผู้อำนวยการฝ่ายวิชาการ. Chanyanat.P@Chula.ac.th 02 218 6274. กลุ่มภารกิจด้านหล`
- **Prefix keyword**: `Some("นางสาวช\u{e31}ญญณ\u{e31}ฐ")`
- **Sub-tokens**: `["น", "าง", "สาว", "ช", "\u{e31}ญ", "ญ", "ณ", "\u{e31}ฐ"]`
- **Primary intent**: `Commercial` (confidence: 79%)

| Keyword | Score |
|---------|-------|
| นางสาวชัญญณัฐ | 1.00 |
| น | 0.80 |
| าง | 0.80 |
| สาว | 0.80 |
| ช | 0.80 |
| ัญ | 0.80 |
| ญ | 0.80 |
| ณ | 0.80 |
| ัฐ | 0.80 |
| ทางเข้า | 0.20 |
| วอเลท | 0.20 |
| เล่น | 0.20 |
| รางวัล | 0.20 |
| สล็อต | 0.20 |

### Sample 272

- **Input**: `น้ำตาลมิตรผล จับมือ มหาวิทยาลัยขอนแก่น ประกาศความร่วมมือทางวิชาการ มุ่งดึงศักยภาพบุคลากร พร้อมต่อยอด`
- **Prefix keyword**: `Some("น\u{e49}ำตาลม\u{e34}ตรผล")`
- **Sub-tokens**: `["น\u{e49}ำ", "ต", "าล", "ม", "\u{e34}ตร", "ผล"]`
- **Primary intent**: `Commercial` (confidence: 53%)

| Keyword | Score |
|---------|-------|
| น้ำตาลมิตรผล | 1.00 |
| น้ำ | 0.80 |
| ต | 0.80 |
| าล | 0.80 |
| ม | 0.80 |
| ิตร | 0.80 |
| ผล | 0.80 |
| 888 | 0.20 |

### Sample 273

- **Input**: `บอล วัน นี้ สปอร์ต พูลยิ่ได้แต้มทุกตา แลกรางวัลพรีเมียม 2026 ระบบใหม่ล่าสุด ทดลองฟรีเลย 2026 ระบบล้ำ`
- **Prefix keyword**: `Some("บอล")`
- **Sub-tokens**: `["บอล"]`
- **Primary intent**: `Commercial` (confidence: 62%)

| Keyword | Score |
|---------|-------|
| บอล | 1.00 |
| ฟรี | 0.60 |
| ทดลอง | 0.40 |
| รางวัล | 0.20 |
| โปรโมชั่น | 0.20 |
| สล็อต | 0.20 |

### Sample 274

- **Input**: `บ้านผลบอล วันนี้ ตารางบอล วันนี้ คืนนี้ ทีเด็ด ฟุตบอล คืนยิ่ดูสูตรสล็อต PG 2026 พร้อมระบบพิเศษ ทดลอง`
- **Prefix keyword**: `Some("บ\u{e49}านผลบอล")`
- **Sub-tokens**: `["บ\u{e49}าน", "ผลบอล"]`
- **Primary intent**: `Commercial` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| บ้านผลบอล | 1.00 |
| บ้าน | 0.80 |
| ผลบอล | 0.80 |
| ฟรี | 0.40 |
| ทดลอง | 0.40 |
| พิเศษ | 0.20 |
| ฝาก | 0.20 |
| ถอน | 0.20 |
| สล็อต | 0.20 |

### Sample 275

- **Input**: `บ้านผลบอลสด goalยิ่แชร์แต้มกับเพื่อน สนุกกว่าเดิม แชร์คะแนนกับเพื่อน ร่วมสนุกฟรี ไม่มีค่าใช้จ่ายใดๆ `
- **Prefix keyword**: `Some("บ\u{e49}านผลบอลสด")`
- **Sub-tokens**: `["บ\u{e49}าน", "ผล", "บอลสด"]`
- **Primary intent**: `Commercial` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| บ้านผลบอลสด | 1.00 |
| บ้าน | 0.80 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ฟรี | 0.60 |
| รางวัล | 0.20 |
| เล่น | 0.20 |
| goal | 0.20 |

### Sample 276

- **Input**: `โบนัส-888 แม้ยังไม่ได้สมัคร คุณก็นสามารถเข้ามาทดลองเล่นเพื่อสัมผัสโลกแห่งการเดิมพันแบบไม่มีความเสี่ย`
- **Prefix keyword**: `Some("โบน\u{e31}ส")`
- **Sub-tokens**: `["โบน\u{e31}ส"]`
- **Primary intent**: `Commercial` (confidence: 54%)

| Keyword | Score |
|---------|-------|
| โบนัส | 1.00 |
| เล่น | 0.40 |
| รูเล็ต | 0.20 |
| ทดลอง | 0.20 |
| สมัคร | 0.20 |
| 888 | 0.20 |
| สล็อต | 0.20 |
| ฟรี | 0.20 |

### Sample 277

- **Input**: `ผล888 เริ่มต้นความสนุกแบบง่ายๆ เพียงแค่ลงทะเบียน คุณก็นำพาตัวเองสู่โลกแห่งการเสิร์ฟที่หลากหลาย ตั้งแ`
- **Prefix keyword**: `Some("ผล")`
- **Sub-tokens**: `["ผล"]`
- **Primary intent**: `Navigational` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| ผล | 1.00 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |
| รูเล็ต | 0.20 |
| เข้าระบบ | 0.20 |
| 888 | 0.20 |

### Sample 278

- **Input**: `ผล-บอล-888-livescore ไม่ต้องโหลดแอป ปลัฟอร์มเว็บไซต์ของเรารองรับมือถือและคอมฯ เล่นได้ทุกหน ทุกแห่ง พ`
- **Prefix keyword**: `Some("ผล")`
- **Sub-tokens**: `["ผล"]`
- **Primary intent**: `Navigational` (confidence: 36%)

| Keyword | Score |
|---------|-------|
| ผล | 1.00 |
| เล่น | 0.40 |
| บาคาร่า | 0.20 |
| livescore | 0.20 |
| สล็อต | 0.20 |
| 888 | 0.20 |

### Sample 279

- **Input**: `ผล​บอล​888 ยิ่งไปกว่านั้น ระบบสมาชิกของเราถูกออกแบบมาเพื่อให้คุณได้รับสิทธิประโยชน์สูงสุด มีของขวัญแ`
- **Prefix keyword**: `Some("ผล")`
- **Sub-tokens**: `["ผล"]`
- **Primary intent**: `Commercial` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| ผล | 1.00 |
| โปรโมชั่น | 0.20 |
| 888 | 0.20 |
| สมาชิก | 0.20 |
| ของขวัญ | 0.20 |

### Sample 280

- **Input**: `ผลบอลgoalยิ่เล่นเกม 2026 ได้คะแนนไม่อั้น ทดลองฟรี ไม่ต้องลงทุน เริ่มเลย! อยากดูสูตรสล็อต PG ก่อนเล่น`
- **Prefix keyword**: `Some("ผลบอล")`
- **Sub-tokens**: `["ผลบอล"]`
- **Primary intent**: `Informational` (confidence: 31%)

| Keyword | Score |
|---------|-------|
| ผลบอล | 1.00 |
| ทดลอง | 0.40 |
| ฟรี | 0.40 |
| เล่น | 0.40 |
| สล็อต | 0.20 |
| goal | 0.20 |

### Sample 281

- **Input**: `ผลบอลพร้อมราคา 888 ล่าสุด อยากลิ้มรสความสนุกที่อยู่ระหว่างปลายนิ้วเพียงแค่คลิก! เพียงลงทะเบียนสมาชิก`
- **Prefix keyword**: `Some("ผลบอลพร\u{e49}อมราคา")`
- **Sub-tokens**: `["ผลบอล", "พร\u{e49}อม", "ราคา"]`
- **Primary intent**: `Navigational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| ผลบอลพร้อมราคา | 1.00 |
| ผลบอล | 0.80 |
| พร้อม | 0.80 |
| ราคา | 0.80 |
| เล่น | 0.40 |
| สล็อต | 0.40 |
| 888 | 0.40 |
| สมัคร | 0.20 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |
| รูเล็ต | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 282

- **Input**: `ผลบอลเมื่อคืน 888 ทุกลีก ภาษาไทย 888 นิ้ว ราคา แบบไร้ขีดจํากัด เว็บไซต์ PG Slot ออนไลน์ยอดนิยมในประเ`
- **Prefix keyword**: `Some("ผลบอลเม\u{e37}\u{e48}อค\u{e37}น")`
- **Sub-tokens**: `["ผลบอล", "เม\u{e37}\u{e48}อค\u{e37}น"]`
- **Primary intent**: `Navigational` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| ผลบอลเมื่อคืน | 1.00 |
| ผลบอล | 0.80 |
| เมื่อคืน | 0.80 |
| 888 | 0.40 |
| เข้าสู่ระบบ | 0.20 |
| pg slot | 0.20 |
| สมาชิก | 0.20 |
| สมัคร | 0.20 |
| slot | 0.20 |

### Sample 283

- **Input**: `ผลบอลเมื่อคืน 888 ทุกลีกยิ่แชร์คะแนนกับเพื่อน สนุกสองเท่า ทดลองเล่นฟรี 2026 คะแนนสะสมแลกของพรีเมียมไ`
- **Prefix keyword**: `Some("ผลบอลเม\u{e37}\u{e48}อค\u{e37}น")`
- **Sub-tokens**: `["ผลบอล", "เม\u{e37}\u{e48}อค\u{e37}น"]`
- **Primary intent**: `Commercial` (confidence: 29%)

| Keyword | Score |
|---------|-------|
| ผลบอลเมื่อคืน | 1.00 |
| ผลบอล | 0.80 |
| เมื่อคืน | 0.80 |
| ฟรี | 0.60 |
| ทดลอง | 0.60 |
| เล่น | 0.20 |
| 888 | 0.20 |

### Sample 284

- **Input**: `ผล บอล เมื่อ คืน 888 ทุก ลีก เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงลงทะเบียน คุณจะได้สัมผัสเกมหลากห`
- **Prefix keyword**: `Some("ผล")`
- **Sub-tokens**: `["ผล"]`
- **Primary intent**: `Transactional` (confidence: 27%)

| Keyword | Score |
|---------|-------|
| ผล | 1.00 |
| สล็อต | 0.20 |
| สมาชิก | 0.20 |
| สมัคร | 0.20 |
| ลงทะเบียน | 0.20 |
| 888 | 0.20 |
| รูเล็ต | 0.20 |

### Sample 285

- **Input**: `ผลบอลเมื่อคืน 888 ทุกลีก อยากลองก่อนตัดสินใจใช่ไหม? ทดลองเล่นฟรีทุกเกมได้ทันที ไม่มีข้อผูกมัด. ไม่ว่`
- **Prefix keyword**: `Some("ผลบอลเม\u{e37}\u{e48}อค\u{e37}น")`
- **Sub-tokens**: `["ผลบอล", "เม\u{e37}\u{e48}อค\u{e37}น"]`
- **Primary intent**: `Commercial` (confidence: 14%)

| Keyword | Score |
|---------|-------|
| ผลบอลเมื่อคืน | 1.00 |
| ผลบอล | 0.80 |
| เมื่อคืน | 0.80 |
| ทดลอง | 0.40 |
| ลงทะเบียน | 0.20 |
| ฟรี | 0.20 |
| ของขวัญ | 0.20 |
| สล็อต | 0.20 |
| เล่น | 0.20 |
| รูเล็ต | 0.20 |
| 888 | 0.20 |

### Sample 286

- **Input**: `ผลบอลเมื่อคืน 888 ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นของขวัญแ`
- **Prefix keyword**: `Some("ผลบอลเม\u{e37}\u{e48}อค\u{e37}น")`
- **Sub-tokens**: `["ผลบอล", "เม\u{e37}\u{e48}อค\u{e37}น"]`
- **Primary intent**: `Commercial` (confidence: 12%)

| Keyword | Score |
|---------|-------|
| ผลบอลเมื่อคืน | 1.00 |
| ผลบอล | 0.80 |
| เมื่อคืน | 0.80 |
| ของขวัญ | 0.20 |
| 888 | 0.20 |
| สมาชิก | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 287

- **Input**: `ผลบอลย้อนหลัง เมื่อคืน ทุกลีก 888 ภาษาไทยยิ่ได้คะแนนพิเศษทุกตา แลกรางวัลฟรี ไม่ต้องเติม อัปเดตโปร 20`
- **Prefix keyword**: `Some("ผลบอลย\u{e49}อนหล\u{e31}ง")`
- **Sub-tokens**: `["ผลบอล", "ย\u{e49}อนหล\u{e31}ง"]`
- **Primary intent**: `Commercial` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| ผลบอลย้อนหลัง | 1.00 |
| ผลบอล | 0.80 |
| ย้อนหลัง | 0.80 |
| ฟรี | 0.40 |
| โบนัส | 0.20 |
| สมัคร | 0.20 |
| รางวัล | 0.20 |
| พิเศษ | 0.20 |
| 888 | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 288

- **Input**: `ผลบอลย้อนหลัง เมื่อคืน ทุกลีก 888 ภาษาไทยยิ่ระบบออโต้เร็ว ทดลองวันนี้เลย อัปเดต 2026 ระบบอัตโนมัติ ฝ`
- **Prefix keyword**: `Some("ผลบอลย\u{e49}อนหล\u{e31}ง")`
- **Sub-tokens**: `["ผลบอล", "ย\u{e49}อนหล\u{e31}ง"]`
- **Primary intent**: `Transactional` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| ผลบอลย้อนหลัง | 1.00 |
| ผลบอล | 0.80 |
| ย้อนหลัง | 0.80 |
| 888 | 0.20 |
| ถอน | 0.20 |
| ฝาก | 0.20 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |

### Sample 289

- **Input**: `ผลบอลวันนี้ 888 คุณก็ได้สัมผัสกับความมันส์ เล่นเกมที่คุณชื่นชอบได้เลย ไม่ต้องโหลดแอป แพลตฟอร์มรองรับ`
- **Prefix keyword**: `Some("ผลบอลว\u{e31}นน\u{e35}\u{e49}")`
- **Sub-tokens**: `["ผลบอล", "ว\u{e31}นน\u{e35}\u{e49}"]`
- **Primary intent**: `Informational` (confidence: 7%)

| Keyword | Score |
|---------|-------|
| ผลบอลวันนี้ | 1.00 |
| ผลบอล | 0.80 |
| วันนี้ | 0.80 |
| เล่น | 0.20 |
| 888 | 0.20 |

### Sample 290

- **Input**: `ผลบอลวันนี้ ทุกลีก ล่าสุด 888ยิ่คะแนนสะสมไม่อั้น แลกของพรีเมียมได้เลย ร่วมสนุก แชร์คะแนน รับรางวัลให`
- **Prefix keyword**: `Some("ผลบอลว\u{e31}นน\u{e35}\u{e49}")`
- **Sub-tokens**: `["ผลบอล", "ว\u{e31}นน\u{e35}\u{e49}"]`
- **Primary intent**: `Commercial` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| ผลบอลวันนี้ | 1.00 |
| ผลบอล | 0.80 |
| วันนี้ | 0.80 |
| รางวัล | 0.40 |
| ฟรี | 0.20 |
| 888 | 0.20 |

### Sample 291

- **Input**: `ผลบอลสด888 livescoreยิ่ทุกการหมุน = คะแนน + ลุ้นรางวัล แชร์คะแนนกับเพื่อน เพิ่มรางวัลทวีคูณ สมัครวัน`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Commercial` (confidence: 38%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ฟรี | 0.60 |
| รางวัล | 0.40 |
| ทดลอง | 0.40 |
| 888 | 0.20 |
| livescore | 0.20 |
| สมัคร | 0.20 |
| โบนัส | 0.20 |

### Sample 292

- **Input**: `ผลบอลสด888 livescore ระบบใหม่ล่าสุด มั่นใจด้วย API ลิขสิทธิ์แท้ พร้อมกิจกรรมรายวันที่อัปเดตอย่างต่อเ`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Navigational` (confidence: 24%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| 888 | 0.40 |
| รางวัล | 0.20 |
| บาคาร่า | 0.20 |
| เล่น | 0.20 |
| พิเศษ | 0.20 |
| สล็อต | 0.20 |
| livescore | 0.20 |

### Sample 293

- **Input**: `ผลบอลสด888 livescore เริ่มต้นความสนุกแบบง่ายๆ แค่กดสมัคร คุณก็จะตื่นตาด้วยเกมหลากหลาย! ทั้งสล็อต, รู`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Navigational` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| 888 | 0.20 |
| รูเล็ต | 0.20 |
| livescore | 0.20 |
| สมัคร | 0.20 |
| สล็อต | 0.20 |

### Sample 294

- **Input**: `ผลบอลสด888 LIVESCORE เล่นกับเว็บตรงคุณภาพระดับโลก การันตีด้วยผู้ใช้งานจริงนับหมื่น ระบบปลอดภัย เสถีย`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Transactional` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ถอน | 0.20 |
| เล่น | 0.20 |
| livescore | 0.20 |
| ฝาก | 0.20 |
| 888 | 0.20 |

### Sample 295

- **Input**: `ผลบอลสด 888 ตารางบอล LIVESCORE 888 โปรแกรมบอลวันนี้ กันได้ เว็บไซต์ PG Slot ออนไลน์ยอดนิยมในประเทศไท`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Navigational` (confidence: 33%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| 888 | 0.40 |
| สมัคร | 0.20 |
| สมาชิก | 0.20 |
| pg slot | 0.20 |
| livescore | 0.20 |
| slot | 0.20 |
| เข้าสู่ระบบ | 0.20 |

### Sample 296

- **Input**: `ผลบอลสด888 เมื่อคืน นอกจากนี้ ทางเราไม่เพียงแค่พัฒนาเกมดีๆ แต่ยังให้คุณ เล่นได้โดยที่ไม่ต้องดาวน์โหล`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Navigational` (confidence: 7%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| เล่น | 0.20 |
| 888 | 0.20 |

### Sample 297

- **Input**: `ผล-บอล-สด-888-วัน-นี้ เริ่มต้นความสนุกแบบจัดเต็ม เพียงแค่คุณลงทะเบียน สู่โลกของเกมหลากหลาย ทั้งสล็อต`
- **Prefix keyword**: `Some("ผล")`
- **Sub-tokens**: `["ผล"]`
- **Primary intent**: `Navigational` (confidence: 23%)

| Keyword | Score |
|---------|-------|
| ผล | 1.00 |
| ทดลอง | 0.20 |
| 888 | 0.20 |
| สล็อต | 0.20 |
| รูเล็ต | 0.20 |
| เล่น | 0.20 |
| ฟรี | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝากเงิน | 0.20 |

### Sample 298

- **Input**: `ผลบอลสดพร้อมราคา 888 พร้อม ราคา เพียงแค่เข้าระบบ เลือกเกมที่ถูกใจทดลองเล่นแบบฟรี ๆ นอกจากนี้ ยังมี ฟ`
- **Prefix keyword**: `Some("ผลบอลสดพร\u{e49}อมราคา")`
- **Sub-tokens**: `["ผล", "บอลสด", "พร\u{e49}อม", "ราคา"]`
- **Primary intent**: `Commercial` (confidence: 8%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดพร้อมราคา | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| พร้อม | 0.80 |
| ราคา | 0.80 |
| เล่น | 0.20 |
| 888 | 0.20 |
| เข้าระบบ | 0.20 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |

### Sample 299

- **Input**: `ผลบอลสดพร้อมราคา 888 พร้อม ราคา เริ่มต้นความสนุกแบบง่าย ๆ เพียงแค่ลงทะเบียน รับสิทธิพิเศษสุด ๆ ตื่นต`
- **Prefix keyword**: `Some("ผลบอลสดพร\u{e49}อมราคา")`
- **Sub-tokens**: `["ผล", "บอลสด", "พร\u{e49}อม", "ราคา"]`
- **Primary intent**: `Navigational` (confidence: 12%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดพร้อมราคา | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| พร้อม | 0.80 |
| ราคา | 0.80 |
| 888 | 0.20 |
| รูเล็ต | 0.20 |
| พิเศษ | 0.20 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 300

- **Input**: `ผลบอลสด ภยิ่ พร้อมความรื่นเริงตลอดการใช้งาน 2026 ล่าสุด ระบบออโต้ดีที่สุด ฟรีไม่มีค่าใช้จ่าย ทดลองระ`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Commercial` (confidence: 19%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |
| รางวัล | 0.20 |

### Sample 301

- **Input**: `ผลบอลสดภาษาไทย 888 ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นแต้มสะส`
- **Prefix keyword**: `Some("ผลบอลสดภาษาไทย")`
- **Sub-tokens**: `["ผล", "บอลสด", "ภาษา", "ไทย"]`
- **Primary intent**: `Transactional` (confidence: 4%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดภาษาไทย | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ภาษา | 0.80 |
| ไทย | 0.80 |
| เล่น | 0.20 |
| โปรโมชั่น | 0.20 |
| 888 | 0.20 |
| สมาชิก | 0.20 |

### Sample 302

- **Input**: `ผลบอลสดภาษาไทย 888 เล่นเกมพนันกับเว็บเรา รับโบนัสทุกยอดฝาก พร้อมระบบฝาก-ถอนอัตโนมัติที่รวดเร็วที่สุด`
- **Prefix keyword**: `Some("ผลบอลสดภาษาไทย")`
- **Sub-tokens**: `["ผล", "บอลสด", "ภาษา", "ไทย"]`
- **Primary intent**: `Commercial` (confidence: 27%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดภาษาไทย | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ภาษา | 0.80 |
| ไทย | 0.80 |
| ฝาก | 0.40 |
| เครดิตฟรี | 0.20 |
| ฟรี | 0.20 |
| โบนัส | 0.20 |
| คืนยอดเสีย | 0.20 |
| เล่น | 0.20 |
| ถอน | 0.20 |
| 888 | 0.20 |

### Sample 303

- **Input**: `ผลบอลสดภาษาไทย thscore ผลบอล 888 livescore ผลบอล สดยิ่ได้คะแนนพิเศษทุกตา แลกรางวัลฟรี ไม่ต้องเติม เร`
- **Prefix keyword**: `Some("ผลบอลสดภาษาไทย")`
- **Sub-tokens**: `["ผล", "บอลสด", "ภาษา", "ไทย"]`
- **Primary intent**: `Commercial` (confidence: 22%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดภาษาไทย | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ภาษา | 0.80 |
| ไทย | 0.80 |
| ฟรี | 0.40 |
| 888 | 0.20 |
| livescore | 0.20 |
| พิเศษ | 0.20 |
| รางวัล | 0.20 |

### Sample 304

- **Input**: `ผลบอลสด ภาษาไทย thscore ผลบอล สด ได้คะแนนพิเศษทุกตา แลกรางวัลฟรี ไม่ต้องเติม สูตรสล็อต PG 2026 ทดลอง`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Commercial` (confidence: 39%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ฟรี | 0.40 |
| พิเศษ | 0.40 |
| รางวัล | 0.20 |
| เล่น | 0.20 |
| ทดลอง | 0.20 |
| โปรโมชั่น | 0.20 |
| ของขวัญ | 0.20 |
| สล็อต | 0.20 |

### Sample 305

- **Input**: `ผลบอลสด ภาษาไทยยิ่ระบบออโต้แท้ ทดลองใช้ฟรี ระบบใหม่ 2026 ทดลองเล่นฟรี ได้คะแนนพิเศษทันที ระบบใหม่ ฝา`
- **Prefix keyword**: `Some("ผลบอลสด")`
- **Sub-tokens**: `["ผล", "บอลสด"]`
- **Primary intent**: `Commercial` (confidence: 41%)

| Keyword | Score |
|---------|-------|
| ผลบอลสด | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| ฟรี | 0.80 |
| ทดลอง | 0.80 |
| ถอน | 0.20 |
| โบนัส | 0.20 |
| เล่น | 0.20 |
| ของขวัญ | 0.20 |
| ฝาก | 0.20 |
| สมาชิก | 0.20 |

### Sample 306

- **Input**: `ผลบอลสดวันนี้ 888ยิ่อัปเดตใหม่ทุกวัน คะแนนสะสมแลกของพรีเมียม คะแนนรอคุณอยู่ ลงทะเบียนเลย 2026 ล่าสุด`
- **Prefix keyword**: `Some("ผลบอลสดว\u{e31}นน\u{e35}\u{e49}")`
- **Sub-tokens**: `["ผล", "บอลสด", "ว\u{e31}นน\u{e35}\u{e49}"]`
- **Primary intent**: `Commercial` (confidence: 10%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดวันนี้ | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| วันนี้ | 0.80 |
| ลงทะเบียน | 0.20 |
| 888 | 0.20 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |

### Sample 307

- **Input**: `ผลบอลสดสำรอง5 เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้งสล็อ`
- **Prefix keyword**: `Some("ผลบอลสดสำรอง")`
- **Sub-tokens**: `["ผล", "บอลสด", "สำ", "รอง"]`
- **Primary intent**: `Commercial` (confidence: 15%)

| Keyword | Score |
|---------|-------|
| ผลบอลสดสำรอง | 1.00 |
| ผล | 0.80 |
| บอลสด | 0.80 |
| สำ | 0.80 |
| รอง | 0.80 |
| สล็อต | 0.60 |
| ทดลอง | 0.40 |
| ลงทะเบียน | 0.40 |
| ฟรี | 0.40 |
| รูเล็ต | 0.20 |
| ถอน | 0.20 |
| เล่น | 0.20 |
| พิเศษ | 0.20 |

### Sample 308

- **Input**: `ผลบอลสำรองยิ่ได้คะแนนพิเศษทุกตา แลกรางวัลฟรี ไม่ต้องเติม 2026 มาแล้ว! ทดลองใช้ระบบใหม่ รับแต้มเพียบ `
- **Prefix keyword**: `Some("ผลบอลสำรองย\u{e34}\u{e48}ได\u{e49}คะแนนพ\u{e34}เศษท\u{e38}กตา")`
- **Sub-tokens**: `["ผลบอล", "สำ", "รอง", "ย", "\u{e34}", "\u{e48}", "ได\u{e49}", "คะแนน", "พ", "\u{e34}เศษ", "ท\u{e38}ก", "ต", "า"]`
- **Primary intent**: `Commercial` (confidence: 91%)

| Keyword | Score |
|---------|-------|
| ผลบอลสำรองยิ่ได้คะแนนพิเศษทุกตา | 1.00 |
| ผลบอล | 0.80 |
| สำ | 0.80 |
| รอง | 0.80 |
| ย | 0.80 |
| ิ | 0.80 |
| ่ | 0.80 |
| ได้ | 0.80 |
| คะแนน | 0.80 |
| พ | 0.80 |
| ิเศษ | 0.80 |
| ทุก | 0.80 |
| ต | 0.80 |
| า | 0.80 |
| ฟรี | 0.40 |
| ทดลอง | 0.40 |
| ไม่ต้องฝาก | 0.20 |
| ขั้นต่ำ | 0.20 |
| เล่น | 0.20 |
| ฝาก | 0.20 |
| พิเศษ | 0.20 |
| รางวัล | 0.20 |

### Sample 309

- **Input**: `ผลหวยเวียดนาม เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณก็จะได้สัมผัสเกมหลากหลาย ไม่ว่า`
- **Prefix keyword**: `Some("ผลหวยเว\u{e35}ยดนาม")`
- **Sub-tokens**: `["ผล", "หวย", "เว", "\u{e35}ยด", "น", "าม"]`
- **Primary intent**: `Transactional` (confidence: 66%)

| Keyword | Score |
|---------|-------|
| ผลหวยเวียดนาม | 1.00 |
| ผล | 0.80 |
| หวย | 0.80 |
| เว | 0.80 |
| ียด | 0.80 |
| น | 0.80 |
| าม | 0.80 |
| สล็อต | 0.20 |
| รูเล็ต | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 310

- **Input**: `ฟุตบอลวันนี้ 888ยิ่ทดลองเล่นสูตรสล็อต PG ฟรี 2026 ระบบใหม่ล่าสุด รับสิทธิพิเศษ แต้มสะสม โปรโมชั่นอัป`
- **Prefix keyword**: `Some("ฟ\u{e38}ตบอลว\u{e31}นน\u{e35}\u{e49}")`
- **Sub-tokens**: `["ฟ\u{e38}ตบอล", "ว\u{e31}นน\u{e35}\u{e49}"]`
- **Primary intent**: `Commercial` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| ฟุตบอลวันนี้ | 1.00 |
| ฟุตบอล | 0.80 |
| วันนี้ | 0.80 |
| ทดลอง | 0.40 |
| ฟรี | 0.40 |
| รางวัล | 0.20 |
| 888 | 0.20 |
| พิเศษ | 0.20 |
| เล่น | 0.20 |
| สล็อต | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 311

- **Input**: `มันสนุกมาก ภาษาอังกฤษ จากการลงทุน คุณสามารถที่จะ ใช้ True Wallet เล่นได้ทุกเกม ไม่ต้องยืนยันบัญชีธนา`
- **Prefix keyword**: `Some("ม\u{e31}นสน\u{e38}กมาก")`
- **Sub-tokens**: `["ม\u{e31}น", "สน", "\u{e38}ก", "มาก"]`
- **Primary intent**: `Informational` (confidence: 4%)

| Keyword | Score |
|---------|-------|
| มันสนุกมาก | 1.00 |
| มัน | 0.80 |
| สน | 0.80 |
| ุก | 0.80 |
| มาก | 0.80 |
| เล่น | 0.20 |
| wallet | 0.20 |

### Sample 312

- **Input**: `ยูฟ่าคลิก ไม่ต้องฝากเงิน เข้าระบบก็เล่นฟรีได้ทันที ทดลองทุกเกม พร้อมสำรวจ ฟีเจอร์ใหม่ล่าสุด ที่อัปเด`
- **Prefix keyword**: `Some("ย\u{e39}ฟ\u{e48}าคล\u{e34}ก")`
- **Sub-tokens**: `["ย\u{e39}", "ฟ", "\u{e48}", "าค", "ล", "\u{e34}ก"]`
- **Primary intent**: `Commercial` (confidence: 86%)

| Keyword | Score |
|---------|-------|
| ยูฟ่าคลิก | 1.00 |
| ยู | 0.80 |
| ฟ | 0.80 |
| ่ | 0.80 |
| าค | 0.80 |
| ล | 0.80 |
| ิก | 0.80 |
| ทดลอง | 0.20 |
| สมาชิก | 0.20 |
| ฝากเงิน | 0.20 |
| เล่น | 0.20 |
| ฝาก | 0.20 |
| ฟรี | 0.20 |
| ไม่ต้องฝาก | 0.20 |
| เข้าระบบ | 0.20 |

### Sample 313

- **Input**: `รวมการ์ตูนบาร์บี้ พลิกแผนร้ายโดรนใจรักพากย์ไทย ส ป พูล ชิโนบุโป้ 30 รับ 100. Your recently viewed it`
- **Prefix keyword**: `Some("รวมการ\u{e4c}ต\u{e39}นบาร\u{e4c}บ\u{e35}\u{e49}")`
- **Sub-tokens**: `["รวม", "การ", "\u{e4c}", "ต", "\u{e39}น", "บ", "าร\u{e4c}", "บ", "\u{e35}\u{e49}"]`
- **Primary intent**: `Transactional` (confidence: 43%)

| Keyword | Score |
|---------|-------|
| รวมการ์ตูนบาร์บี้ | 1.00 |
| รวม | 0.80 |
| การ | 0.80 |
| ์ | 0.80 |
| ต | 0.80 |
| ูน | 0.80 |
| บ | 0.80 |
| าร์ | 0.80 |
| ี้ | 0.80 |

### Sample 314

- **Input**: `โรงพยาบาลตํารวจ เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้งสล`
- **Prefix keyword**: `Some("โรงพยาบาลต\u{e4d}ารวจ")`
- **Sub-tokens**: `["โรง", "พ", "ย", "าบ", "าล", "ต", "\u{e4d}", "าร", "วจ"]`
- **Primary intent**: `Informational` (confidence: 35%)

| Keyword | Score |
|---------|-------|
| โรงพยาบาลตํารวจ | 1.00 |
| โรง | 0.80 |
| พ | 0.80 |
| ย | 0.80 |
| าบ | 0.80 |
| าล | 0.80 |
| ต | 0.80 |
| ํ | 0.80 |
| าร | 0.80 |
| วจ | 0.80 |
| สล็อต | 0.60 |
| ทดลอง | 0.20 |
| ถอน | 0.20 |
| เล่น | 0.20 |
| ขั้นต่ำ | 0.20 |
| ฝาก | 0.20 |
| slot | 0.20 |
| รูเล็ต | 0.20 |

### Sample 315

- **Input**: `ลิงก์เข้าเล่น ซุปเปอร์ สล็อต777 สล็อตเว็บตรง อัปเดตล่าสุดปี 2026 สำหรับผู้ที่ต้องการระบบเสถียรและปลอ`
- **Prefix keyword**: `Some("ล\u{e34}งก\u{e4c}เข\u{e49}าเล\u{e48}น")`
- **Sub-tokens**: `["ล", "\u{e34}ง", "ก", "\u{e4c}", "เข\u{e49}า", "เล\u{e48}น"]`
- **Primary intent**: `Transactional` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| ลิงก์เข้าเล่น | 1.00 |
| ล | 0.80 |
| ิง | 0.80 |
| ก | 0.80 |
| ์ | 0.80 |
| เข้า | 0.80 |
| เล่น | 0.80 |
| สล็อต | 0.60 |

### Sample 316

- **Input**: `เลขที่ 43 ม.2 ตำบลโพนงาม อำเภอ คำชะอี จังหวัดมุกดาหาร 49110. โทรศัพท์ / โทรสาร 042-664500. sarabun@p`
- **Prefix keyword**: `Some("เลขท\u{e35}\u{e48}")`
- **Sub-tokens**: `["เลข", "ท\u{e35}\u{e48}"]`
- **Primary intent**: `Unknown` (confidence: 0%)

| Keyword | Score |
|---------|-------|
| เลขที่ | 1.00 |
| เลข | 0.80 |
| ที่ | 0.80 |

### Sample 317

- **Input**: `เว็บfafa ไม่ว่าคุณจะชอบสล็อต PG สนุกกับระบบสปิน หรืออยากดื่มด่ำความมันส์ของเกมโป๊กเกอร์ มีให้ครบ พร้`
- **Prefix keyword**: `Some("เว\u{e47}บ")`
- **Sub-tokens**: `["เว\u{e47}บ"]`
- **Primary intent**: `Informational` (confidence: 11%)

| Keyword | Score |
|---------|-------|
| เว็บ | 1.00 |
| ของขวัญ | 0.20 |
| ทดลอง | 0.20 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 318

- **Input**: `เว็บfafa ระบบสมาชิกออกแบบมาเพื่อให้เข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นของขวัญแต้มสะสม หรือ`
- **Prefix keyword**: `Some("เว\u{e47}บ")`
- **Sub-tokens**: `["เว\u{e47}บ"]`
- **Primary intent**: `Informational` (confidence: 18%)

| Keyword | Score |
|---------|-------|
| เว็บ | 1.00 |
| วิธี | 0.20 |
| โปรโมชั่น | 0.20 |
| เล่น | 0.20 |
| สมาชิก | 0.20 |
| ของขวัญ | 0.20 |
| สมัคร | 0.20 |

### Sample 319

- **Input**: `เว็บดูหนังออนไลน์ฟรี 24 ชั่วโมงพากย์ไทย อยากดูสูตรสล็อต PG ก่อนเล่นจริง? ทดลองระบบอัตโนมัติฟรี ไม่มี`
- **Prefix keyword**: `Some("เว\u{e47}บด\u{e39}หน\u{e31}งออนไลน\u{e4c}ฟร\u{e35}")`
- **Sub-tokens**: `["เว\u{e47}บ", "ด\u{e39}", "หน\u{e31}ง", "ออนไลน\u{e4c}", "ฟร\u{e35}"]`
- **Primary intent**: `Commercial` (confidence: 45%)

| Keyword | Score |
|---------|-------|
| เว็บดูหนังออนไลน์ฟรี | 1.00 |
| เว็บ | 0.80 |
| ดู | 0.80 |
| หนัง | 0.80 |
| ออนไลน์ | 0.80 |
| ฟรี | 0.80 |
| ทดลอง | 0.40 |
| เล่น | 0.20 |
| สล็อต | 0.20 |

### Sample 320

- **Input**: `เว็บออนไลน์888 ระบบใหม่ล่าสุด มั่นใจด้วย API ลิขสิทธิ์แท้ พร้อมกิจกรรมรายวันที่อัปเดตอย่างต่อเนื่อง `
- **Prefix keyword**: `Some("เว\u{e47}บออนไลน\u{e4c}")`
- **Sub-tokens**: `["เว\u{e47}บ", "ออนไลน\u{e4c}"]`
- **Primary intent**: `Commercial` (confidence: 12%)

| Keyword | Score |
|---------|-------|
| เว็บออนไลน์ | 1.00 |
| เว็บ | 0.80 |
| ออนไลน์ | 0.80 |
| เล่น | 0.20 |
| รางวัล | 0.20 |
| พิเศษ | 0.20 |
| 888 | 0.20 |

### Sample 321

- **Input**: `ศูนย์บริการวิชาการแห่งจุฬาลงกรณ์มหาวิทยาลัย ทุกสัปดาห์ เว็บตรง รองรับการเข้าถึงผ่าน Android และ iOS `
- **Prefix keyword**: `Some("ศ\u{e39}นย\u{e4c}บร\u{e34}การว\u{e34}ชาการแห\u{e48}งจ\u{e38}ฬาลงกรณ\u{e4c}มหาว\u{e34}ทยาล\u{e31}ย")`
- **Sub-tokens**: `["ศ", "\u{e39}น", "ย\u{e4c}", "บร\u{e34}การ", "ว\u{e34}", "ช", "าการ", "แห\u{e48}ง", "จ", "\u{e38}", "ฬ", "าล", "ง", "กรณ\u{e4c}", "มห", "าว\u{e34}", "ทยาล\u{e31}ย"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| ศูนย์บริการวิชาการแห่งจุฬาลงกรณ์มหาวิทยาลัย | 1.00 |
| ศ | 0.80 |
| ูน | 0.80 |
| ย์ | 0.80 |
| บริการ | 0.80 |
| วิ | 0.80 |
| ช | 0.80 |
| าการ | 0.80 |
| แห่ง | 0.80 |
| จ | 0.80 |
| ุ | 0.80 |
| ฬ | 0.80 |
| าล | 0.80 |
| ง | 0.80 |
| กรณ์ | 0.80 |
| มห | 0.80 |
| าวิ | 0.80 |
| ทยาลัย | 0.80 |
| สล็อต | 0.40 |

### Sample 322

- **Input**: `ศูนย์วิจัยและพัฒนาการเกษตรพัทลุง. หน้าหลัก · เกี่ยวกับ ศวพ.พัทลุง · ประวัติ ... Slot Nedir. Slot Ned`
- **Prefix keyword**: `Some("ศ\u{e39}นย\u{e4c}ว\u{e34}จ\u{e31}ยและพ\u{e31}ฒนาการเกษตรพ\u{e31}ทล\u{e38}ง")`
- **Sub-tokens**: `["ศ", "\u{e39}น", "ย\u{e4c}", "ว\u{e34}", "จ", "\u{e31}ย", "และ", "พ", "\u{e31}ฒ", "น", "าการ", "เก", "ษ", "ตร", "พ", "\u{e31}ท", "ล", "\u{e38}ง"]`
- **Primary intent**: `Commercial` (confidence: 51%)

| Keyword | Score |
|---------|-------|
| ศูนย์วิจัยและพัฒนาการเกษตรพัทลุง | 1.00 |
| ศ | 0.80 |
| ูน | 0.80 |
| ย์ | 0.80 |
| วิ | 0.80 |
| จ | 0.80 |
| ัย | 0.80 |
| และ | 0.80 |
| พ | 0.80 |
| ัฒ | 0.80 |
| น | 0.80 |
| าการ | 0.80 |
| เก | 0.80 |
| ษ | 0.80 |
| ตร | 0.80 |
| ัท | 0.80 |
| ล | 0.80 |
| ุง | 0.80 |
| slot | 0.60 |
| เล่น | 0.20 |
| ทดลอง | 0.20 |
| สมาชิก | 0.20 |
| โปรโมชั่น | 0.20 |
| ของขวัญ | 0.20 |
| สล็อต | 0.20 |

### Sample 323

- **Input**: `สนาม กีฬา กลาง จังหวัด มุกดาหาร เริ่มต้นความมันส์เพียงแค่คลิก! ลงทะเบียนกับเรา คุณจะเข้าสู่โลกของเกม`
- **Prefix keyword**: `Some("สนาม")`
- **Sub-tokens**: `["สนาม"]`
- **Primary intent**: `Informational` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| สนาม | 1.00 |
| เล่น | 0.40 |
| รางวัล | 0.20 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |

### Sample 324

- **Input**: `สนุก888 นับตั้งแต่วินาทีแรกที่เข้าสู่ระบบ คุณสามารถเลือกทดลองเล่นเกมในหมวดที่ชอบเลย แม้ยังไม่ได้สมัค`
- **Prefix keyword**: `Some("สน\u{e38}ก")`
- **Sub-tokens**: `["สน", "\u{e38}ก"]`
- **Primary intent**: `Informational` (confidence: 14%)

| Keyword | Score |
|---------|-------|
| สนุก | 1.00 |
| สน | 0.80 |
| ุก | 0.80 |
| ทดลอง | 0.40 |
| สมัคร | 0.20 |
| เข้าสู่ระบบ | 0.20 |
| เล่น | 0.20 |
| ของขวัญ | 0.20 |
| 888 | 0.20 |
| ลงทะเบียน | 0.20 |

### Sample 325

- **Input**: `สนุกมาก-ภาษาอังกฤษ ระบบสมาชิกของเราออกแบบมา เพื่อมอบสิทธิประโยชน์ให้คุณอย่างเต็มที่ ไม่ว่าจะเป็นของข`
- **Prefix keyword**: `Some("สน\u{e38}กมาก")`
- **Sub-tokens**: `["สน", "\u{e38}ก", "มาก"]`
- **Primary intent**: `Commercial` (confidence: 25%)

| Keyword | Score |
|---------|-------|
| สนุกมาก | 1.00 |
| สน | 0.80 |
| ุก | 0.80 |
| มาก | 0.80 |
| รางวัล | 0.20 |
| ฟรี | 0.20 |
| ฝาก | 0.20 |
| ถอน | 0.20 |
| ของขวัญ | 0.20 |
| สมาชิก | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 326

- **Input**: `สมัครสอบตำรวจ เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้งสล็อ`
- **Prefix keyword**: `Some("สม\u{e31}ครสอบตำรวจ")`
- **Sub-tokens**: `["สม\u{e31}คร", "สอบ", "ตำรวจ"]`
- **Primary intent**: `Transactional` (confidence: 65%)

| Keyword | Score |
|---------|-------|
| สมัครสอบตำรวจ | 1.00 |
| สมัคร | 0.80 |
| สอบ | 0.80 |
| ตำรวจ | 0.80 |
| รูเล็ต | 0.20 |
| ฝาก | 0.20 |
| เข้าระบบ | 0.20 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝากเงิน | 0.20 |

### Sample 327

- **Input**: `สมาคมลับคูคลักซ์แคลน ; ISBN. 978-616-389-170-9 ; หน่วยงานจัดพิมพ์. [1] สำนักงานราชบัณฑิตยสภา ; แหล่ง`
- **Prefix keyword**: `Some("สมาคมล\u{e31}บค\u{e39}คล\u{e31}กซ\u{e4c}แคลน")`
- **Sub-tokens**: `["สม", "าคม", "ล", "\u{e31}บ", "ค", "\u{e39}", "คล", "\u{e31}ก", "ซ\u{e4c}", "แ", "คล", "น"]`
- **Primary intent**: `Transactional` (confidence: 86%)

| Keyword | Score |
|---------|-------|
| สมาคมลับคูคลักซ์แคลน | 1.00 |
| สม | 0.80 |
| าคม | 0.80 |
| ล | 0.80 |
| ับ | 0.80 |
| ค | 0.80 |
| ู | 0.80 |
| คล | 0.80 |
| ัก | 0.80 |
| ซ์ | 0.80 |
| แ | 0.80 |
| น | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 328

- **Input**: `สลอด888 | โบนัสคืนเงิน ที่ดีลสุดคุ้ม. สลอด888 แนะนำ เกมกระดานยอดฮิต สำหรับผู้ที่สนใจ เกมเดิมพันสายบั`
- **Prefix keyword**: `Some("สลอด")`
- **Sub-tokens**: `["ส", "ลอด"]`
- **Primary intent**: `Transactional` (confidence: 65%)

| Keyword | Score |
|---------|-------|
| สลอด | 1.00 |
| ส | 0.80 |
| ลอด | 0.80 |
| 888 | 0.60 |
| สมัคร | 0.20 |
| แนะนำ | 0.20 |
| เล่น | 0.20 |
| โบนัส | 0.20 |

### Sample 329

- **Input**: `สล็อต888 pg เริ่มต้นความสนุกแบบไม่มีขีดจำกัด เพียงแค่ลงทะเบียนคุณก็สัมผัสกับโลกของเกมคาสิโนและกีฬาได`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Navigational` (confidence: 53%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| เล่น | 0.60 |
| 888 | 0.40 |
| ฟรี | 0.20 |
| ไม่ต้องฝาก | 0.20 |
| ฝากเงิน | 0.20 |
| ฝาก | 0.20 |
| รูเล็ต | 0.20 |

### Sample 330

- **Input**: `สล็อต-888-เว็บตรงไม่ผ่านเอเย่นต์ไม่มีขั้นต่ํา เกมคาสิโนและกีฬาในที่เดียว เลือกเล่นได้ตามสไตล์คุณ พร้`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Navigational` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| ไม่ต้องฝาก | 0.20 |
| 888 | 0.20 |
| สมัคร | 0.20 |
| ฝาก | 0.20 |
| พิเศษ | 0.20 |
| ฟรี | 0.20 |
| ของขวัญ | 0.20 |

### Sample 331

- **Input**: `สล็อต888เว็บตรง ระบบสมาชิกออกแบบมาเพื่อให้คุณเข้าถึงสิทธิประโยชน์อย่างเต็มที่ ไม่ว่าจะเป็นของขวัญแต้`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Navigational` (confidence: 58%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| ของขวัญ | 0.20 |
| ฟรี | 0.20 |
| 888 | 0.20 |
| รูเล็ต | 0.20 |
| เล่น | 0.20 |
| สมาชิก | 0.20 |
| สมัคร | 0.20 |

### Sample 332

- **Input**: `สล็อต888เว็บตรง เริ่มต้นความสนุกแบบไม่ต้องคิดเยอะ เพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมหลากหลาย ทั้งสล`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Navigational` (confidence: 70%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| ฝาก | 0.20 |
| รูเล็ต | 0.20 |
| ลงทะเบียน | 0.20 |
| ฝากเงิน | 0.20 |
| 888 | 0.20 |

### Sample 333

- **Input**: `สล็อต 888 อยากรู้เพิ่มเติม เข้ามาเพื่อทดลองเล่นฟรีทุกเกม พร้อมสิทธิ์สะสมแต้มแลกรางวัล หรือเลือกประเภ`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Navigational` (confidence: 58%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| เล่น | 0.40 |
| รูเล็ต | 0.20 |
| 888 | 0.20 |
| ฟรี | 0.20 |
| ทดลอง | 0.20 |
| รางวัล | 0.20 |

### Sample 334

- **Input**: `สล็อตfafa เว็บสล็อตมาแรง รวมสล็อตแตกง่ายจากค่ายใหญ่ ฝากถอนเร็ว ไม่มีขั้นต่ำ รองรับทรูวอลเล็ต.gam ggย`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Transactional` (confidence: 53%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| ทดลอง | 0.60 |
| ฝาก | 0.40 |
| ถอน | 0.40 |
| ฟรี | 0.40 |
| ขั้นต่ำ | 0.20 |
| พิเศษ | 0.20 |
| เล่น | 0.20 |

### Sample 335

- **Input**: `สล็อต ggยิ่อัปเดต 2026 ระบบอัตโนมัติ ฝากถอนไม่มีสะดุด คะแนนสะสมไม่อั้น แลกของพรีเมียมได้ทุกวัน ระบบส`
- **Prefix keyword**: `Some("สล\u{e47}อต")`
- **Sub-tokens**: `["สล\u{e47}อต"]`
- **Primary intent**: `Transactional` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| สล็อต | 1.00 |
| สมาชิก | 0.20 |
| ฟรี | 0.20 |
| ฝาก | 0.20 |
| ถอน | 0.20 |
| ทดลอง | 0.20 |

### Sample 336

- **Input**: `สล็อตเครดิตฟรี ไม่ต้องฝาก ไม่ต้องแชร์ ระบบสมาชิกที่ออกแบบเฉพาะ เพื่อความสะดวกในการเข้าถึงสิทธิ์พิเศษ`
- **Prefix keyword**: `Some("สล\u{e47}อตเครด\u{e34}ตฟร\u{e35}")`
- **Sub-tokens**: `["สล\u{e47}อต", "เครด\u{e34}ตฟร\u{e35}"]`
- **Primary intent**: `Commercial` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| สล็อตเครดิตฟรี | 1.00 |
| สล็อต | 0.80 |
| เครดิตฟรี | 0.80 |
| ฝาก | 0.20 |
| พิเศษ | 0.20 |
| สมาชิก | 0.20 |
| ไม่ต้องฝาก | 0.20 |
| ฟรี | 0.20 |
| โปรโมชั่น | 0.20 |

### Sample 337

- **Input**: `สล็อตยูฟ่าเว็บตรง ระบบสมาชิกของเราออกแบบมาเพื่อให้ประสบการณ์การเล่นของคุณเต็มอิ่ม ไม่ว่าจะเป็น "ของข`
- **Prefix keyword**: `Some("สล\u{e47}อตย\u{e39}ฟ\u{e48}าเว\u{e47}บตรง")`
- **Sub-tokens**: `["สล\u{e47}อต", "ย\u{e39}", "ฟ\u{e48}า", "เว\u{e47}บ", "ตรง"]`
- **Primary intent**: `Navigational` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สล็อตยูฟ่าเว็บตรง | 1.00 |
| สล็อต | 0.80 |
| ยู | 0.80 |
| ฟ่า | 0.80 |
| เว็บ | 0.80 |
| ตรง | 0.80 |
| ของขวัญ | 0.20 |
| สมาชิก | 0.20 |
| เล่น | 0.20 |

### Sample 338

- **Input**: `สล็อตรับโปรโมชั่น พร้อมกันนั้น แพลตฟอร์มของเรายังมีจุดเด่นอีกอย่างนั่นคือ ระบบสมาชิกที่ออกแบบมาเพื่อ`
- **Prefix keyword**: `Some("สล\u{e47}อตร\u{e31}บโปรโมช\u{e31}\u{e48}น")`
- **Sub-tokens**: `["สล\u{e47}อต", "ร\u{e31}บ", "โปรโมช\u{e31}\u{e48}น"]`
- **Primary intent**: `Commercial` (confidence: 55%)

| Keyword | Score |
|---------|-------|
| สล็อตรับโปรโมชั่น | 1.00 |
| สล็อต | 0.80 |
| รับ | 0.80 |
| โปรโมชั่น | 0.80 |
| สมาชิก | 0.20 |
| ของขวัญ | 0.20 |
| รางวัล | 0.20 |

### Sample 339

- **Input**: `สสจ.พัทลุง เว็บสล็อตรวมค่าย สมัครใช้งานง่าย เข้าสู่ระบบได้ทันที ระบบลื่นไหล เล่นสะดวก มีโอกาสลุ้นราง`
- **Prefix keyword**: `Some("สสจ")`
- **Sub-tokens**: `["ส", "ส", "จ"]`
- **Primary intent**: `Commercial` (confidence: 94%)

| Keyword | Score |
|---------|-------|
| สสจ | 1.00 |
| ส | 0.80 |
| จ | 0.80 |
| เล่น | 0.20 |
| เข้าสู่ระบบ | 0.20 |
| สล็อต | 0.20 |
| สมัคร | 0.20 |
| รางวัล | 0.20 |

### Sample 340

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ Aผลบอลสด ในปี แพลตฟอร์มที่น่าเชื่อถือ ครบครันทั้งบาคา`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 341

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ดู บอล สด วัน นี้ ไทย ระดับประเทศ แพลตฟอร์มที่น่าเชื่`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 31%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.40 |
| สล็อต | 0.40 |

### Sample 342

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ดูหนังXXฟรี จะได้รับ แพลตฟอร์มที่น่าเชื่อถือ ครบครันท`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 34%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| ฟรี | 0.20 |
| บาคาร่า | 0.20 |

### Sample 343

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ตรวจหวยถ่ายทอดสดหวย ทุกค่าย แพลตฟอร์มที่น่าเชื่อถือ ค`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 344

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ตรวจหวยรัฐบาลวันที่ 1 มิถุนายน 2561 ก็ แพลตฟอร์มที่น่`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 345

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ตรวจ หวย เวียดนาม วัน นี้ ไม่ แพลตฟอร์มที่น่าเชื่อถือ`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 346

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ตารางบอลพรีเมียร์ลีกแมนยู เล่น แพลตฟอร์มที่น่าเชื่อถื`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 31%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| เล่น | 0.20 |
| สล็อต | 0.20 |

### Sample 347

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ตาราง บอล วัน นี้ พรีเมียร์ ลีก ส่งตรงจากต่างประเทศ แ`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 348

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ บอล วัน นี้ พรีเมียร์ ลีก มืออาชีพ แพลตฟอร์มที่น่าเชื`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 349

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ บอลอังกฤษสด โดย แพลตฟอร์มที่น่าเชื่อถือ ครบครันทั้งบา`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 350

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ บ้าน ผล บอล เมื่อ คืน ทุก ลีก ถือเป็น แพลตฟอร์มที่น่า`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 351

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ โปรแกรม ถ่ายทอด สด ฟุตบอล ตาราง บอล วัน นี้ พรีเมียร์`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 352

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ โปรแกรม บอล พรุ่งนี้ พรีเมียร์ ลีก ได้เลย แพลตฟอร์มที`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 353

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ผล บอล ล่าสุด พรีเมียร์ ลีก อังกฤษ เข้ามาวางแผนการเดิ`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 354

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ ผลบอลสด888วันนี้ อีกเพียบ แพลตฟอร์มที่น่าเชื่อถือ ครบ`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 31%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| 888 | 0.20 |
| สล็อต | 0.20 |
| บาคาร่า | 0.20 |

### Sample 355

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ รายงาน ผล บอล หลายคน แพลตฟอร์มที่น่าเชื่อถือ ครบครันท`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 356

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ วิเคราะห์บอลยูฟ่าแชมป์เปียนลีก ที่เปิดให้บริการ แพลตฟ`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 32%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| บาคาร่า | 0.20 |
| สล็อต | 0.20 |

### Sample 357

- **Input**: `สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ หวย ผล สลากกินแบ่ง รัฐบาล งวด 2563 เพียง แพลตฟอร์มที่`
- **Prefix keyword**: `Some("ส\u{e31}มผ\u{e31}สประสบการณ\u{e4c}เด\u{e34}มพ\u{e31}นออนไลน\u{e4c}ระด\u{e31}บพร\u{e35}เม\u{e35}ยมก\u{e31}บ")`
- **Sub-tokens**: `["ส\u{e31}ม", "ผ", "\u{e31}ส", "ประ", "สบ", "การณ\u{e4c}", "เด\u{e34}มพ\u{e31}น", "ออนไลน\u{e4c}", "ระด\u{e31}บ", "พ", "ร\u{e35}เม", "\u{e35}ย", "ม", "ก\u{e31}บ"]`
- **Primary intent**: `Commercial` (confidence: 30%)

| Keyword | Score |
|---------|-------|
| สัมผัสประสบการณ์เดิมพันออนไลน์ระดับพรีเมียมกับ | 1.00 |
| สัม | 0.80 |
| ผ | 0.80 |
| ัส | 0.80 |
| ประ | 0.80 |
| สบ | 0.80 |
| การณ์ | 0.80 |
| เดิมพัน | 0.80 |
| ออนไลน์ | 0.80 |
| ระดับ | 0.80 |
| พ | 0.80 |
| รีเม | 0.80 |
| ีย | 0.80 |
| ม | 0.80 |
| กับ | 0.80 |
| สล็อต | 0.40 |
| joker | 0.20 |
| บาคาร่า | 0.20 |
| bet | 0.20 |

### Sample 358

- **Input**: `สำรวจความสนุกของ PG SLOT DEMO พร้อมระบบซื้อฟรีสปินและโหมดทดลองเล่นครบทุกเกม เรียนรู้รูปแบบการเล่น เท`
- **Prefix keyword**: `Some("สำรวจความสน\u{e38}กของ")`
- **Sub-tokens**: `["สำ", "รวจ", "ความ", "สน", "\u{e38}ก", "ของ"]`
- **Primary intent**: `Commercial` (confidence: 17%)

| Keyword | Score |
|---------|-------|
| สำรวจความสนุกของ | 1.00 |
| สำ | 0.80 |
| รวจ | 0.80 |
| ความ | 0.80 |
| สน | 0.80 |
| ุก | 0.80 |
| ของ | 0.80 |
| เล่น | 0.40 |
| slot | 0.20 |
| ทดลอง | 0.20 |
| ฟรี | 0.20 |
| pg slot | 0.20 |
| สล็อต | 0.20 |

### Sample 359

- **Input**: `หนังโป๊เปิดซิง เริ่มต้นความสนุกที่ง่ายดายเพียงแค่ลงทะเบียน คุณจะได้เข้าถึงเกมหลากหลายไม่ว่าจะเป็นสล็`
- **Prefix keyword**: `Some("หน\u{e31}งโป\u{e4a}เป\u{e34}ดซ\u{e34}ง")`
- **Sub-tokens**: `["หน\u{e31}ง", "โป", "\u{e4a}", "เป\u{e34}ด", "ซ", "\u{e34}ง"]`
- **Primary intent**: `Commercial` (confidence: 13%)

| Keyword | Score |
|---------|-------|
| หนังโป๊เปิดซิง | 1.00 |
| หนัง | 0.80 |
| โป | 0.80 |
| ๊ | 0.80 |
| เปิด | 0.80 |
| ซ | 0.80 |
| ิง | 0.80 |
| สล็อต | 0.20 |
| ลงทะเบียน | 0.20 |
| รูเล็ต | 0.20 |

### Sample 360

- **Input**: `หวย ดัง พัทลุง   สล็อตแตกง่าย แจกหนักจัดเต็ม โปรสล็อตสุดคุ้ม เข้าร่วมโปรโมชั่นพิเศษ ผู้ชื่นชอบเกมมือ`
- **Prefix keyword**: `Some("หวย")`
- **Sub-tokens**: `["หวย"]`
- **Primary intent**: `Commercial` (confidence: 50%)

| Keyword | Score |
|---------|-------|
| หวย | 1.00 |
| สล็อต | 0.60 |
| ฟรี | 0.40 |
| โบนัส | 0.20 |
| ทดลอง | 0.20 |
| แจก | 0.20 |
| โปรโมชั่น | 0.20 |
| พิเศษ | 0.20 |
| เล่น | 0.20 |

### Sample 361

- **Input**: `หวยเวียดนามวันนี้>Visit the PlayStation Store. Platform : PlayStation 4. 4.2 4.2 out of 5 stars 394 `
- **Prefix keyword**: `Some("หวยเว\u{e35}ยดนามว\u{e31}นน\u{e35}\u{e49}")`
- **Sub-tokens**: `["หวย", "เว", "\u{e35}ยด", "น", "าม", "ว\u{e31}นน\u{e35}\u{e49}"]`
- **Primary intent**: `Informational` (confidence: 69%)

| Keyword | Score |
|---------|-------|
| หวยเวียดนามวันนี้ | 1.00 |
| หวย | 0.80 |
| เว | 0.80 |
| ียด | 0.80 |
| น | 0.80 |
| าม | 0.80 |
| วันนี้ | 0.80 |
| วิธี | 0.20 |
| เล่น | 0.20 |
| สล็อต | 0.20 |

### Sample 362

- **Input**: `หวยเวียดนามวันนี้ อยากรู้ว่าแล้ว จะเล่นฟรี หรือเลือกระบบสะสมแต้มไปแลกรางวัล? อยากลองก่อนตัดสินใจใช่ไ`
- **Prefix keyword**: `Some("หวยเว\u{e35}ยดนามว\u{e31}นน\u{e35}\u{e49}")`
- **Sub-tokens**: `["หวย", "เว", "\u{e35}ยด", "น", "าม", "ว\u{e31}นน\u{e35}\u{e49}"]`
- **Primary intent**: `Informational` (confidence: 66%)

| Keyword | Score |
|---------|-------|
| หวยเวียดนามวันนี้ | 1.00 |
| หวย | 0.80 |
| เว | 0.80 |
| ียด | 0.80 |
| น | 0.80 |
| าม | 0.80 |
| วันนี้ | 0.80 |
| รางวัล | 0.20 |
| ฟรี | 0.20 |
| เล่น | 0.20 |

### Sample 363

- **Input**: `... อำเภอคำชะอี จังหวัดมุกดาหาร. ... คำค้นหา : kggสล็อต(1-88.vip)เกมเยอะที่สุดในไทย สมัครรับ 500บาท `
- **Prefix keyword**: `Some("อำเภอคำชะอ\u{e35}")`
- **Sub-tokens**: `["อ", "ำเภ", "อ", "คำ", "ช", "ะ", "อ", "\u{e35}"]`
- **Primary intent**: `Informational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| อำเภอคำชะอี | 1.00 |
| อ | 0.80 |
| ำเภ | 0.80 |
| คำ | 0.80 |
| ช | 0.80 |
| ะ | 0.80 |
| ี | 0.80 |
| vip | 0.20 |
| สมัคร | 0.20 |
| สล็อต | 0.20 |

### Sample 364

- **Input**: `... อำเภอคำชะอี จังหวัดมุกดาหาร. ... สล็อตเว็บตรง,(16lv.com) LV68 คาสิโนออนไลน์ดีที่สุดในไทย ครบจบใน`
- **Prefix keyword**: `Some("อำเภอคำชะอ\u{e35}")`
- **Sub-tokens**: `["อ", "ำเภ", "อ", "คำ", "ช", "ะ", "อ", "\u{e35}"]`
- **Primary intent**: `Informational` (confidence: 100%)

| Keyword | Score |
|---------|-------|
| อำเภอคำชะอี | 1.00 |
| อ | 0.80 |
| ำเภ | 0.80 |
| คำ | 0.80 |
| ช | 0.80 |
| ะ | 0.80 |
| ี | 0.80 |
| สล็อต | 0.20 |

### Sample 365

- **Input**: `เฮงๆ888 เริ่มต้นความสนุกแบบไร้ขั้นตอนเพียงแค่ลงทะเบียน คุณจะได้สัมผัสเกมคาสิโน และกีฬากับสล็อต PG แล`
- **Prefix keyword**: `Some("เฮงๆ")`
- **Sub-tokens**: `["ง", "ๆ"]`
- **Primary intent**: `Transactional` (confidence: 81%)

| Keyword | Score |
|---------|-------|
| เฮงๆ | 1.00 |
| ง | 0.80 |
| ๆ | 0.80 |
| ฟรี | 0.20 |
| เข้าระบบ | 0.20 |
| เล่น | 0.20 |
| ทดลอง | 0.20 |
| ขั้นตอน | 0.20 |
| 888 | 0.20 |
| รูเล็ต | 0.20 |
| สล็อต | 0.20 |

---

## Sample JSON Output

```json
{
  "primary_intent": "Commercial",
  "confidence_score": 0.29,
  "extracted_keywords": [
    {
      "word": "1000",
      "score": 1.0
    },
    {
      "word": "100",
      "score": 0.8
    },
    {
      "word": "พิเศษ",
      "score": 0.4
    },
    {
      "word": "รางวัล",
      "score": 0.2
    },
    {
      "word": "vip",
      "score": 0.2
    },
    {
      "word": "สมาชิก",
      "score": 0.2
    },
    {
      "word": "ลงทะเบียน",
      "score": 0.2
    },
    {
      "word": "โปรโมชั่น",
      "score": 0.2
    },
    {
      "word": "ของขวัญ",
      "score": 0.2
    }
  ],
  "metadata": {
    "language": "th",
    "processor": "rake-rs-v1"
  }
}
```

---

## Performance

- **100 iterations**: `33.544093ms`
- **Average per iteration**: `0.33ms` (target: <50ms)
- **Status**: PASS ✓
