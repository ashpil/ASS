# Project Charter

## Logistics

We will communicate through text, Zoom, and in-person meetings. We will collaborate using git and GitHub.

## Proposal

Web development has changed a lot since 1997. Although the tools have evolved overtime, they’re still fundamentally designed to solve the problems of 1997. At that time, the goal was the separation of content structure and styling. HTML allowed users to define content structure while freeing independent consumers to apply custom stylesheets. This is no longer the case. In today’s world, frontend developers write HTML, CSS, and JavaScript. Many CSS-in-JS frameworks allow the user to specify content and styling as one whole piece. As a result, the principles of separating style and content structure have become much more contrived. In many instances, styling has become content, and we believe that the tools should express that intent.

Our goal is to develop a language which will allow a developer to specify both the content and styling of a web page. The syntax of this language should imply the union of styling and content. As an example, a button’s identity would no longer be a textfield with styling. The styling would become part of the button’s identity. This language would take inspiration from popular frontend frameworks such as Tailwind, Svelte and React and try to natively implement what JavaScript has been coerced into doing for years.

With this language, we would also like to integrate constraint styling. In Badros’ 1999 paper entitled, Constraint Cascading Style Sheets for the Web, a framework for style sheets based on hierarchical constraints was proposed that would, “...provide a unifying, declarative semantics for CSS 2.0 and also suggest a simplifying implementation strategy.” Despite the touted benefits of constraint CSS, the idea never took hold in the web development community, but this idea did influence Apple’s Cocoa AutoLayout for iOS, which proved this constraint model to be highly effective.

To summarize, our project would involve designing and building a language to unify content and styling. We would also like to take the opportunity to improve the way styling is performed in the browser with constraint styling.

An example of a possible syntax for our language (no constraints):

```
bundle par, do: text_color(black) font_size(16) justify no_wrap
bundle tag(color), do: background(color) border_color(darken(color, 10%)) inline

<center big>
	<par inline>Welcome to my post!</par>
<tag(red)>Media</tag>
</center>
```

## Alpha - Language Specification and Proof of Concept

- Written representation of syntax and semantics
- Generate lexer and parser from written representation using already-built libraries
- Modify or create a minimal browser framework so that it can process parts of our language
- Implement pre-built tags of the language such that a page with simple styles can be generated and painted in our modified browser

## Beta - Implement Core Features of the Language

- Implement basic language constructs such as functions and variables to allow users to build their own tags
- Using an existing constraints solving library such as Cassowary, implement constraint features of the language

## Final - Add on Extra Features, Polishing

- Squash bugs
- Implement any intended features that were not finished in Beta
- If we have additional time, implement nice-to-have tooling features such as syntax highlighting

## Preliminary Design

The syntax and semantics of our programming language will be formalized. Ideally, we would like to design comfortable syntax that reduces boilerplate and code duplication when compared to HTML and CSS.

We will write this project in Rust, which is great for its memory safety and high performance. There are also many similar projects to base our project on. Additionally, we anticipate that we will have to deal with many tree data structures and algorithms in order to construct a render tree which can then be handled by the browser. We will also need to obtain a thorough understanding of constraints solving, so that we can effectively employ a constraint solving library.

This project will utilize Rust’s module system. We tentatively intend to separate the project into modules as follows:

- Language interpreter
  - Parsing and lexing
  - Execution/element creation
- Displaying
  - Positioning elements
  - Painting elements

We will try to use existing third-party libraries whenever possible. For example, there is an existing Cassowary Constraint Solver implementation in Rust. There are also many options for parsing and lexing which we can choose from. Since Mozilla’s browser engine, Servo, is highly modular and written in Rust, many rendering features are already implemented and available for reuse.

Rust also has an advanced unit testing system. We will employ test-driven development. For the visual side of things, Insta is a good Rust snapshot testing library. We will use git branches to implement features incrementally while keeping them backed up, and only merge PRs with tests.
