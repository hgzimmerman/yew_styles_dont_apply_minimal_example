# Demonstrates a bug with yew as of Yew 8.0.

Yew does not update styles on the first re-render. 2 updates need to occur in order for style changes to appear.

You can observe this in this project by running it with `cargo-web start`, and navigating to the address where it is hosted.
From there, clicking the "Set Red" button once, not noticing a change, and then clicking that button again, or the "Rerender" button will cause the intended update to the styles.