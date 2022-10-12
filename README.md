#### Example:

I want to evaluate both paspio and zxcvbn crates on rockyou.txt wordlist with a minimum zxcvbn score of 3/4 (zxcvbn ranks your passwords from 0 to 4)

```bash
$ ./dirty-entropy-rs rockyou.txt 3
```

#### Result:

```
'teamomiamor' has an entropy of 51.70483689955201 (paspio) or 8.143712248016522 (zxcvbn) and a score of 3
'tekieromucho' has an entropy of 56.405276617693104 (paspio) or 10.73319726510657 (zxcvbn) and a score of 4
'212224236' has an entropy of 29.89735285398626 (paspio) or 8.060697840353612 (zxcvbn) and a score of 3
'estrelinha' has an entropy of 47.004397181410916 (paspio) or 10.00000000004343 (zxcvbn) and a score of 3
'ciocolata' has an entropy of 42.303957463269825 (paspio) or 9.000000000434294 (zxcvbn) and a score of 3
'ichliebedich' has an entropy of 56.405276617693104 (paspio) or 12.000000000000433 (zxcvbn) and a score of 4
'tekieromuxo' has an entropy of 51.70483689955201 (paspio) or 9.740362689494244 (zxcvbn) and a score of 3
'florecita' has an entropy of 42.303957463269825 (paspio) or 8.368119458615029 (zxcvbn) and a score of 3
'tekelomuxo' has an entropy of 47.004397181410916 (paspio) or 10.00000000004343 (zxcvbn) and a score of 3
'chaparrita' has an entropy of 47.004397181410916 (paspio) or 8.062474641944668 (zxcvbn) and a score of 3
...
```