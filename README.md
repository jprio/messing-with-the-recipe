What if we learned just enough to be a little dangerous?
Ecriture en rust d'un .so utilisé avec LD_PRELOAD pour odifier le comportement d'un programme (cat)

https://fasterthanli.me/videos/messing-with-the-recipe

>cargo run 

>file target/debug/messing-with-the-recipe.exe

>hexyl -n 256 target/debug/messing-with-the-recipe
>nm target/debug/messing-with-the-recipe | grep main
>objdump --disassemble=_ZN23messing_with_the_recipe4main17hb2397b379aaf7e8bE -M intel target/debug/messing-with-the-recipe