# Tailwind CSS formatting

Please style the tailwind classes as follows, to keep them orgainized.

```jsx
<div className="
    border sm:border-2
    self-end

    text-center md:text-right
    font-bold
    text-xl sm:text-2xl lg:text-4xl
    break-words

    col-start-4
    col-span-6 sm:col-span-8 md:col-span-full

    row-span-1
    md:row-start-2
  "
>
Hello World
<div />
```

If the same property is set for multiple breakpoints, then keep them on the same line. Order them from the smallest screen size upward. Like this: `p-2 sm:p-1.5 md:p-2 lg:p-3`. Do the same for pseudo classes (:hover, :active). `shadow-sm hover:shadow-md` are on the same line because they are the same property.

Separate the properties with a new line by rough categories: text styles, position, flex properties... etc.

This does increase the scroll length of the code, but it does make it more readable and editable. This is a compromise.

_Exception_: if there are few classes, and the column width is less than 80 characters, then don't use multi-line. Use discretion.

If you are using VIM, this is a quick way to convert a single line to multi lines:

1. Visual select the inner quotes using `vi"`
1. Run `:s/ /\r/g` to replace spaces with new lines

# Contributing

Work is done on a project basis, not on a task basis.

People like to have autonomy while doing creative work.

A single git issue + feature branch will contain the 'project'
Or use github projects??? Hah.

# Comments

- `BUG:` for bugs
- `TODO:` for todos
- `NOTE:` for notes

This makes them easy to search for.

# Feature Notes

## Profile Page
There are 3 main profile states: your own profile, a following profile and a not-following profile. There should be a logout button somewhere.

## How to page
There is 'owned by me' then there is 'owned by someone else'.
'owned by me' allows for creating and updating content.

## Star feature
How important is the 'star' feature for this to take off?
Future: There a button to 'star' a How To, with a number of total stars next to it.
Future: There is a page for all the How-tos that you've stared.

## Shared editing feature
More than one person can modify the same how to

## Following / Followers feature
A person may follow zero or more other people
Zero or more people may follow that person

## Sign up / sign in feature

## Password reset feature
