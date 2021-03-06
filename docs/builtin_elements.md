# Builtin Elements

## Common properties

These properties are valid on all visible items

* **`x`** and **`y`** (*length*): the position of the element relative to its parent
* **`width`** and **`height`** (*length*): The size of the element. When set, this overrides the default size.
* **`maximum_width`** and **`maximum_height`** (*length*): The maximum size of an element when used in a layout.
* **`minimum_width`** and **`minimum_height`** (*length*): The minimum size of an element when used in a layout.
* **`col`**, **`row`**, **`colspan`**, **`rowspan`** (*int*): See [`GridLayout`](#gridlayout).

## `Window`

Window is the root of what is on the screen


## `Rectangle`

By default, the rectangle is just an empty item that shows nothing. By setting a color or a border
it is then possible to draw a simple rectangle on the screen

### Properties

* **`color`** (*color*): The background color of the Rectangle. (default value: transparent)
* **`border_width`** (*length*): The width of the border. (default value: 0)
* **`border_color`** (*color*): The color of the border. (default value: transparent)
* **`border_radius`** (*length*): The size of the radius. (default value: 0)

### Example

```60
Example := Window {
    width: 270px;
    height: 100px;

    Rectangle {
        x: 10px;
        y: 10px;
        width: 50px;
        height: 50px;
        color: blue;
    }

    // Rectangle with a border
    Rectangle {
        x: 70px;
        y: 10px;
        width: 50px;
        height: 50px;
        color: green;
        border_width: 2px;
        border_color: red;
    }

    // Transparent Rectangle with a border and a radius
    Rectangle {
        x: 140px;
        y: 10px;
        width: 50px;
        height: 50px;
        border_width: 4px;
        border_color: black;
        border_radius: 10px;
    }

    // A radius of width/2 makes it a circle
    Rectangle {
        x: 210px;
        y: 10px;
        width: 50px;
        height: 50px;
        color: yellow;
        border_width: 2px;
        border_color: blue;
        border_radius: width/2;
    }
}
```

## `Image`

An Image can be used to represent an image loaded from an image file

### Properties

* **`source`** (*image*): The image to load. In order to reference image, one uses the `img!"..."` macro
  which loads the file relative to the directory containing the .60 file.

### Example


```60
Example := Image {
    source: img!"https://raw.githubusercontent.com/sixtyfpsui/sixtyfps/master/resources/logo_scaled.png";
    width: 64px;
    height: 44px;
}
```

## `Text`

A text simply show the text on the screen

### Properties

* **`text`** (*string*): The actual text.
* **`font_family`** (*string*): The font name
* **`font_size`** (*length*): The font size of the text
* **`color`** (*color*): The color of the text (default: transparent)
* **`horizontal_alignment`**, **`vertical_alignment`** (*FIXME: enum*): How is the text aligned
  within the item



### Example

```60
Example := Window {
    width: 270px;
    height: 100px;

    Text {
        text: "Hello World";
        color: red;
    }
}
```

## `Path`

FIXME: write docs

## `TouchArea`

The TouchArea control what happens when the zone covered by it is touched or interacted with the mouse.

### Properties

* **`pressed`** (*bool*): Set to true by the TouchArea when the mouse is pressed over it.
* **`mouse_x`**, **`mouse_y`** (*length*): Set by the TouchArea to the position of the mouse within it.
* **`pressed_x`**, **`mouse_y`** (*length*): Set to true by the TouchArea to the position of the
    mouse at the moment it was last pressed.

### Signals

* **`clicked`**: Emited when the mouse is released

### Example

```60
Example := Window {
    width: 200px;
    height: 100px;
    area := TouchArea {
        width: parent.width;
        height: parent.height;
        clicked => {
            rect2.color = #ff0;
        }
    }
    Rectangle {
        width: parent.width / 2;
        height: parent.height;
        color: area.pressed ? blue: red;
    }
    rect2 := Rectangle {
        x: parent.width / 2;
        width: parent.width / 2;
        height: parent.height;
    }
}
```

## `GridLayout`

`GridLayout` places the elements in a grid. `GridLayout` adds properties to each item: `col`, `row`, `colspan`, `rowspan`.
You can control the position of elements with `col` and `row`.
If `col` or `row` is not specified, they are automatically computed such that the item is next to the previous item, in the same row.
Alternatively, the item can be put in a `Row` element.

### Properties

 * **`spacing`** (*length*): The distance between the elements in the layout.
 * **`padding`** (*length*): the padding within the layout.
 * **`padding_left`**, **`padding_right`**, **`padding_top`** and **`padding_bottom`** (*length*):
    override the padding in specific sides.

### Examples

This example use the `Row` element

```60
Foo := Window {
    width: 200px;
    height: 200px;
    GridLayout {
        spacing: 5px;
        Row {
            Rectangle { color: red; }
            Rectangle { color: blue; }
        }
        Row {
            Rectangle { color: yellow; }
            Rectangle { color: green; }
        }
    }
}
```

This example use the `col` and `row` property

```60
Foo := Window {
    width: 200px;
    height: 150px;
    GridLayout {
        Rectangle { color: red; }
        Rectangle { color: blue; }
        Rectangle { color: yellow; row: 1; }
        Rectangle { color: green; }
        Rectangle { color: black; col: 2; row: 0; }
    }
}
```

## `PathLayout`

FIXME: write docs

## `Flickable`

FIXME: write docs

## `TextInput`

The `TextInput` is a lower-level item that shows text and allows entering text.

### Properties

* **`text`** (*string*): The actual text.
* **`font_family`** (*string*): The font name
* **`font_size`** (*length*): The font size of the text
* **`color`** (*color*): The color of the text (default: transparent)
* **`horizontal_alignment`**, **`vertical_alignment`** (*FIXME: enum*): How is the text aligned
  within the item
* **`has_focus`** (*bool*): Set to true when item is focused and receives keyboard events.

### Methods

* **`focus()`** Call this function to focus the text input and make it receive future keyboard events.

### Example

```60
Example := Window {
    width: 270px;
    height: 100px;

    TextInput {
        text: "Replace me with a name";
    }
}
```


