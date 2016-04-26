# Taxo

Taxo is a little command line utility that takes as input a rules file and a
path. It evaluates the rules and returns the value of the first rule that
matches the path.

The original motivation was to enable the automatic sorting of code files into
the appropriate editor (i.e. app vs. test).

Rule files look like this:

```
g **/*_test.rs test
g src/**.rs app
g test/**.rs test
```

`g` in the above stands for "glob", then the pattern itself, and finally the
value of the rule.

The can get more complicated, especially since rules can be expressed with
regular expressions, and both glob and regex rules can have options set.
