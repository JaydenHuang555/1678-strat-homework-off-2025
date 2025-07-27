valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ./cycles example.jsonc 2>&1 | tee valgrind-out.txt
