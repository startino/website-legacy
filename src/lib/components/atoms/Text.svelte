<script lang="ts">
	enum TextType {
		Display,
		Headline,
		Title,
		Label,
		Body
	}
	enum TextSize {
		Large,
		Medium,
		Small
	}

	export let textType;
	export let textSize;
	/*
    let text = document.getElementById("text");

    switch(textType) {
        case TextType.Display:
            text?.classList.add("text-6xl");
            break;
        case TextType.Headline:
            text?.classList.add("text-2xl");
            break;
        case TextType.Title:
            text?.classList.add("text-lg");
            break;
        case TextType.Label:
            text?.classList.add("font-bold");
            break;
        case TextType.Body:
            text?.classList.add("text-base");
            break;
    }

    switch(textSize) {
        case TextSize.Large:
            break;
        case TextSize.Medium:
            break;
        case TextSize.Small:
            break;
    }*/
</script>

<div id="text" class="{$$props.class}">
	<slot />
</div>
