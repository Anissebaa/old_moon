================================================================================
                        THE OLD MOON PUBLIC LICENSE
                             Version 0.1.42, 2025
================================================================================

Copyright (c) 2026, Somebody Who Was Probably Tired

Everyone is permitted to copy and distribute verbatim or modified copies of
this license document, and changing it is allowed as long as the name is
changed. Changing the name is not allowed. Wait. Scratch that. Do whatever
you want. We're not your lawyer.

--------------------------------------------------------------------------------
                           TERMS AND CONDITIONS
--------------------------------------------------------------------------------

0. DEFINITIONS

   "This Software"     refers to old_moon, including all Rust, Lua, Python,
                       and Ruby code contained herein, and any confusion
                       arising therefrom.

   "You"               refers to the person reading this, the person running
                       `sudo ./old_moon`, and/or the person who caused the
                       segfault. Probably all three.


   "The me ig.."       refers to whoever wrote this. They are not available
                       for comment. They are also not sure how the Ruby
                       engine works.

   "A Packet"          refers to a unit of network traffic. You will capture
                       many of these. Some of them will be your own. Do not
                       be alarmed.

1. GRANT OF RIGHTS

   Permission is hereby granted, free of charge, to any person obtaining a
   copy of This Software and associated documentation files (the "Software"),
   to deal in the Software without restriction, including without limitation
   the rights to use, copy, modify, merge, publish, distribute, sublicense,
   and/or sell copies of the Software, subject to the following conditions:

   a) You must retain the above copyright notice and this permission notice
      in all copies or substantial portions of the Software.

   b) You must not claim to understand how all four languages interact at
      runtime. Nobody does. Not even the compiler. Especially not the
      compiler.

   c) If you run this on a network you do not own, that is between you and
      your network administrator, your ISP, your conscience, and possibly
      a judge. The Authors have no opinion on this and no legal standing in
      the matter. Good luck.

2. THE "IT WORKS ON MY MACHINE" CLAUSE

   This Software has been observed to function on at least one machine, at
   least once, at least for a few minutes, under at least one phase of the
   moon. Results may vary. The Authors make no guarantee that it will:

   a) compile
   b) run
   c) capture packets
   d) not capture packets it shouldn't
   e) produce output you understand
   f) not summon a daemon, literal or otherwise
   g) behave consistently across Lua 5.3 and Lua 5.4
   h) work at all if you have recently said "this should be easy"

3. THE "ONLY GOD UNDERSTANDS THIS CODE" CLAUSE

   By using This Software, you acknowledge and agree that:

   a) The interaction between Rust's borrow checker, Lua's garbage
      collector, Python's GIL, and Ruby's everything is not fully
      documented anywhere on Earth.

   b) The Author(s) have, at various points, been surprised by their own
      code doing exactly what it was written to do.

   c) Any bug you find is a feature. Any feature you find is a bug. Any
      comment you find has been removed per project requirements.

   d) If you fix something, you must not tell anyone how, because then
      someone will ask you to fix something else.

4. THE "PACKET CAPTURE AND PRIVILEGE" CLAUSE

   This Software requires elevated privileges to capture network traffic.
   You are expected to know what that means. The Authors are not responsible
   for:

   a) Anything you capture that you were not supposed to see.
   b) Anything you capture that you were supposed to see but wish you hadn't.
   c) Anything you fail to capture because you forgot `sudo` again.
   d) The legal, professional, personal, and/or spiritual consequences of
      running a packet sniffer on a network belonging to someone with
      lawyers.

5. THE "GUI MODE" CLAUSE

   The `--gui` flag does not provide a graphical user interface. It shells
   out to `tshark` or `pyshark`, which are Wireshark's problem, not ours.
   If you wanted a GUI, you should have installed Wireshark. If you installed
   Wireshark and still ran this, we appreciate the sentiment but question
   the decision.

6. THE "RUBY ENGINE" CLAUSE

   The Ruby engine exists. It compiles. On some systems. Under certain
   conditions. The Authors make no further claims about it. If it works for
   you, please open an issue and describe your environment in detail,
   because nobody else can reproduce it.

7. THE "COMMENT-FREE ZONE" CLAUSE

   This codebase contains no explanatory comments. This was a deliberate
   design decision made by someone who was not consulted. If you find
   yourself confused, please refer to Clause 3, Section (a).

8. WARRANTY DISCLAIMER

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE, NONINFRINGEMENT, OR THE ABILITY TO
   COMPILE ON A FRESH CHECKOUT WITHOUT EDITING THE CARGO.TOML FILE FIRST.

   IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
   CLAIM, DAMAGES, OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
   TORT, OR OTHERWISE, ARISING FROM, OUT OF, OR IN CONNECTION WITH THE
   SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE — INCLUDING BUT
   NOT LIMITED TO:

   a) Data loss
   b) Packet loss
   c) Will to live
   d) Suspicious `if let Some(...)` chains that shouldn't have compiled
   e) A Lua script that filters out everything, including the packets
      you actually wanted
   f) That one time `tshark` wasn't in your PATH

9. TERMINATION

   This license terminates automatically if you:

   a) Claim to have read and understood all of `main.rs`
   b) Assert that Ruby and Python should coexist in the same binary
   c) Ask why there are no comments

   Upon termination, you must destroy all copies of the Software in your
   possession, and also apologize to the compiler.

10. FINAL CLAUSE

    By running This Software, you agree that:

    - You did this to yourself.
    - The moon was old when you started, and it will be older when you stop.
    - Somewhere, somehow, a packet you captured was somebody else's problem.
    - Only God understands this code.

================================================================================
                     END OF TERMS AND CONDITIONS
================================================================================
================================================================================
                        رخصة القمر العجوز العامة
                             الإصدار 1.0، 2026
================================================================================

حقوق النشر (c) 2026، شخصٌ كان على الأرجح متعباً

يُسمح للجميع بنسخ وتوزيع نسخ حرفية أو معدّلة من وثيقة الرخصة هذه،
ويُسمح بتغييرها ما دام الاسم قد تغيّر. تغيير الاسم غير مسموح. انتظر.
تراجع عن ذلك. افعل ما تريد. نحن لسنا محاميك.

--------------------------------------------------------------------------------
                           الشروط والأحكام
--------------------------------------------------------------------------------

0. التعريفات

   "هذا البرنامج"      يشير إلى old_moon، بما في ذلك كل شيفرة Rust و Lua
                       و Python و Ruby الواردة هنا، وأي ارتباك ينشأ عنها.

   "أنت"               يشير إلى الشخص الذي يقرأ هذا، والشخص الذي يشغّل
                       `sudo ./old_moon`، و/أو الشخص الذي تسبّب في
                       الـ segfault. على الأرجح أنت الثلاثة معاً.

   "الله"              يشير إلى الكائن الوحيد الذي يُفهم أنه يستوعب
                       قاعدة الشيفرة هذه بالكامل. انظر README.md.

   "المؤلفون"          يشيرون إلى من كتب هذا. غير متاحين للتعليق.
                       وهم أيضاً غير متأكدين من كيفية عمل محرك Ruby.

   "حزمة"              تشير إلى وحدة من حركة الشبكة. ستلتقط الكثير منها.
                       بعضها سيكون خاصتك. لا تنزعج.

1. منح الحقوق

   يُمنح الإذن بموجب هذا، مجاناً، لأي شخص يحصل على نسخة من هذا البرنامج
   والملفات التوثيقية المصاحبة (يُشار إليها بـ "البرنامج")، بالتعامل مع
   البرنامج دون قيود، بما في ذلك على سبيل المثال لا الحصر حقوق الاستخدام
   والنسخ والتعديل والدمج والنشر والتوزيع والترخيص الفرعي و/أو البيع،
   وفقاً للشروط التالية:

   أ) يجب عليك الاحتفاظ بإشعار حقوق النشر أعلاه وإشعار الإذن هذا في جميع
      النسخ أو الأجزاء الجوهرية من البرنامج.

   ب) يجب ألا تدّعي فهم كيفية تفاعل اللغات الأربع أثناء التشغيل. لا أحد
      يفهمها. ولا حتى المترجم. خصوصاً المترجم.

   ج) إذا شغّلت هذا على شبكة لا تملكها، فهذا شأنك أنت ومدير شبكتك ومزوّد
      خدمة الإنترنت وضميرك، وربما قاضٍ. لا رأي للمؤلفين في هذا ولا صفة
      قانونية لهم في الأمر. حظاً موفقاً.

2. بند "يعمل على جهازي"

   لوحظ أن هذا البرنامج يعمل على جهاز واحد على الأقل، مرة واحدة على
   الأقل، لبضع دقائق على الأقل، تحت طور واحد على الأقل من أطوار القمر.
   النتائج قد تختلف. لا يقدّم المؤلفون أي ضمان بأنه سوف:

   أ) يُترجم (compile)
   ب) يعمل (run)
   ج) يلتقط حزماً
   د) لا يلتقط حزماً لا ينبغي له التقاطها
   هـ) يُنتج مخرجات تفهمها
   و) لا يستدعي شيطاناً، حرفياً أو مجازياً
   ز) يتصرف بشكل متسق بين Lua 5.3 و Lua 5.4
   ح) يعمل أصلاً إذا قلت مؤخراً "هذا يجب أن يكون سهلاً"

3. بند "الله وحده يفهم هذه الشيفرة"

   باستخدامك لهذا البرنامج، فإنك تقرّ وتوافق على أن:

   أ) التفاعل بين مدقّق الاستعارة في Rust، وجامع القمامة في Lua،
      وقفل المفسّر العام في Python، وكل شيء في Ruby، غير موثّق بالكامل
      في أي مكان على وجه الأرض.

   ب) فوجئ المؤلف (المؤلفون)، في مراحل مختلفة، بشيفرتهم تفعل بالضبط
      ما كُتبت لتفعله.

   ج) أي خطأ تجده هو ميزة. أي ميزة تجدها هي خطأ. أي تعليق تجده قد أُزيل
      وفقاً لمتطلبات المشروع.

   د) إذا أصلحت شيئاً، يجب ألا تخبر أحداً كيف، لأن شخصاً ما سيسألك حينها
      إصلاح شيء آخر.

4. بند "التقاط الحزم والصلاحيات"

   يتطلب هذا البرنامج صلاحيات مرتفعة لالتقاط حركة الشبكة. يُفترض أنك
   تعرف ماذا يعني ذلك. المؤلفون غير مسؤولين عن:

   أ) أي شيء تلتقطه لم يكن يفترض بك رؤيته.
   ب) أي شيء تلتقطه كان يفترض بك رؤيته لكنك تتمنى لو لم تره.
   ج) أي شيء تفشل في التقاطه لأنك نسيت `sudo` مرة أخرى.
   د) العواقب القانونية والمهنية والشخصية و/أو الروحية لتشغيلك
      ماسح حزم على شبكة يملكها شخص لديه محامون.

5. بند "وضع الواجهة الرسومية"

   علَم `--gui` لا يوفّر واجهة رسومية. إنه يستدعي `tshark` أو `pyshark`،
   وهذه مشكلة Wireshark، لا مشكلتنا. إذا أردت واجهة رسومية، كان عليك
   تثبيت Wireshark. إذا ثبّت Wireshark وما زلت تشغّل هذا، فنحن نقدّر
   العاطفة لكننا نتساءل عن القرار.

6. بند "محرك Ruby"

   محرك Ruby موجود. يُترجم. على بعض الأنظمة. تحت ظروف معينة. لا يقدّم
   المؤلفون أي ادعاءات إضافية بشأنه. إذا عمل لديك، فيرجى فتح تذكرة
   ووصف بيئتك بالتفصيل، لأن لا أحد آخر يستطيع إعادة إنتاجه.

7. بند "منطقة خالية من التعليقات"

   لا تحتوي قاعدة الشيفرة هذه على تعليقات توضيحية. كان هذا قراراً
   تصميمياً متعمداً اتخذه شخص لم يُستَشَر. إذا وجدت نفسك مرتبكاً،
   يرجى الرجوع إلى البند 3، الفقرة (أ).

8. إخلاء الضمان

   يُقدَّم البرنامج "كما هو"، دون أي ضمان من أي نوع، صريح أو ضمني، بما
   في ذلك على سبيل المثال لا الحصر ضمانات القابلية للتسويق، أو الملاءمة
   لغرض معين، أو عدم التعدي، أو القدرة على الترجمة من نسخة جديدة دون
   تعديل ملف Cargo.toml أولاً.

   لا يكون المؤلفون أو أصحاب حقوق النشر مسؤولين في أي حال عن أي مطالبة
   أو أضرار أو مسؤولية أخرى، سواء في دعوى عقدية أو تقصيرية أو غير ذلك،
   الناشئة عن أو المرتبطة بالبرنامج أو استخدامه أو أي تعاملات أخرى فيه
   — بما في ذلك على سبيل المثال لا الحصر:

   أ) فقدان البيانات
   ب) فقدان الحزم
   ج) الرغبة في العيش
   د) سلاسل `if let Some(...)` المشبوهة التي لم يكن يفترض بها أن تُترجم
   هـ) سكربت Lua يرشّح كل شيء، بما في ذلك الحزم التي أردتها فعلاً
   و) تلك المرة التي لم يكن فيها `tshark` في مسار PATH لديك

9. الإنهاء

   تنتهي هذه الرخصة تلقائياً إذا:

   أ) ادّعيت أنك قرأت وفهمت كل ملف `main.rs`
   ب) أكّدت أن Ruby و Python يجب أن يتعايشا في نفس الملف التنفيذي
   ج) سألت لماذا لا توجد تعليقات

   عند الإنهاء، يجب عليك تدمير جميع نسخ البرنامج في حوزتك، والاعتذار
   أيضاً للمترجم.

10. البند الأخير

    بتشغيلك لهذا البرنامج، فإنك توافق على أن:

    - أنت فعلت هذا بنفسك.
    - كان القمر عجوزاً عندما بدأت، وسيكون أقدم عندما تتوقف.
    - في مكان ما، بطريقة ما، كانت حزمة التقطتها مشكلة شخص آخر.
    - الله وحده يفهم هذه الشيفرة.

================================================================================
                     نهاية الشروط والأحكام
================================================================================
