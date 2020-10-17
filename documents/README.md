# ASS Language Specification

## Data Types

ASS understands a lot of common web data types by default.

- *Pixel* values are denoted by `px`, as in `16px`.
- *Percentage* values are denoted by `%`, as in `75%`.
- *Color hex* values are denoted by `#`, as in `#7fffd4`.

## Definitions

**Trait**
A trait is a word indicating a stylistic quality of a block - for example it's size, position or color.
A trait can be a *simple trait*, meaning it takes no arguments, or a *complex trait* meaning it takes at least one argument.
Traits can take arguments in brackets - `trait[arg]`.

**Tag**
A tag is a sequence of one or more traits in angle brackets. There are closing and opening tags.
An opening tag takes the form of `<traits>` and a closing tag takes the form `</traits>`.
A tag can also be both closing and opening at the same time `<traits/>`.

**Block**
A block is delimited by an opening and closing tag. The insides of the tag indicate a blocks contents, while the traits inside the opening tag indicate its traits.
A block can have many children blocks, and one parent.

## Content Tree
```ass
<center tallest>
    <par inline>Welcome to my post!</par>
    <tag["red"]>Media</tag>
</center>
```

The content tree is a recursive block structure. Each pair of opening-closing tags can have blocks within them.


## Trait Bundles
```ass
par = text_color["black"] font_size[16px] justify no_wrap
tag[color] = bg_color[color] border_color[darken(color, 10%)]
```

If you have a lot of blocks that need the `text_color["black"] font_size[16px] justify no_wrap` traits, writing them out every time might start feeling repatative.

Worry not! You can bundle them together so that one small word means all of that.

Trait bundles are typically used to specify properties that are irrelevant to the positioning of an element - such as color or roundedness.

## Constraints 

```ass
body require {
    width <= 80rem
    hcenter = $window.center
} prefer {
    width = 80rem
}

title require {
    hcenter = $window.center
    top = $parent.top + 1rem
}
```

And finally, constraints! Using them you can define a responsive layout that eliminates the edge cases and always looks good!

The above code ensures the `body` always has a width less than 80rem, and if possible, just keeps it at 80 rem. 
It also makes sure the center of the body is the center of the window.
This means that on small screens the body will fill up the entire screen, and on super large screens, it won't get bigger than 80 rem. Neat!

The `title` part ensures titles are always centered in the window, and that the top of a title is `1rem` below the top of the parent container. 
Special variables, such as `$parent` or `$window` are designated by `$`.

