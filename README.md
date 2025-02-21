# BEVY_AUTO_SCALING

A very simple plugin for bevy to let graphics auto rescale and center with the window.

![image](img.png)

Normally, the view of a camera will be rendered to the whole window without scaling, so when the resolution changes, some unintended behavior will happen. For `Camera3d`, the scale will change, revealing or hiding some objects; for `Camera2d`, the resolution of the window directly determines the resolution of the camera, and changing it will mess up the display.

With this plugin, the display of cameras can be automatically rescaled and centered with the change of the window, while keeping a fixed aspect ratio, as the image above displays. Moreover, although **bevy** has `OrthographicProjection` to give `Camera2d` a fixed resolution, the plugin offers a function to easily create it, and make the camera remain a fixed aspect ratio rather than filling the entire window.

Therefore, the users no longer have to concern on the size of window, but can focus on the "logistic" graphic itself.


# Usage

To auto-scale a camera, first, you need to add `ScalePlugin` to the `App`. 

Then, you need to add `AspectRatio` component to the camera entity. 

To fix the resolution of a `Camera2d`, you can put function `fixed_size_2d` in the camera buldle. It will return an `OrthographicProjection` component.