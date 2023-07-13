RustyTerm changelog

- v0.1.0:
---
Project created

**Commands available in v0.1.0:**

- ls
- cd
- cat
- mkdir
- touch
- pwd
- rm
- rmdir
- exit

- v0.1.1:
---
Code has been **refactored**, some functions were merged to make a **more readable and maintinable code**. Also, there are **new commands available** in RustyTerminal, here is the full list:

- sudo
- apt
- cp
- mv
- man
- chmod
- unzip
- echo
- ps
- kill
- tail
- head

Our target for next version is to have some **keyboard shortcuts** like using the **horizontal arrows to move on the prompt** or the **vertical arrows to move between you command history**, also he want to include the **use of pipes and redirections** as well as some **other commands that are not ready yet**.

- v0.1.2:
---
There's been a big update on the input bar, now you can use the arrow to move through the written text! Use the left and right arrow to move horizontally through the text.

Also, we are introducing **command history!** Use the upper arrow to auto type the previously introduced commands increasing your speed on the terminal exponentially.

**No new commands had been added to v0.1.2, next version will come with more**

For the future we want to introduce **auto completion** as well as **syntax highlighting** for the input line, as well as increasing the number of commands available on the terminal.

In a more technical view, we keep refactoring the code, specifically the match command argument, we are grouping the commands that use the same functions so it keeps easy to maintain and with a high legibility.

- v0.1.3:
---
Being able to edit text or code in the terminal is so important, can you use a terminal without a text editor included? I can't! For that reason we are including nano and vim into the list of available commands.

Here you have the list of the new commands available in RustyTerm:

- Nano
- Vim

Also, we have now available the official documentation of the project at docs.rs/

-v0.1.4:
---
In v0.1.4 we are including pipes!! Now don't be scared of using pipes becasuse they work perfectly! 