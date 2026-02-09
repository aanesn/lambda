<script lang="ts">
	import { Avatar } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: Avatar.ImageProps = $props()
</script>

<Avatar.Image class={cx("aspect-square size-full", className)} {...restProps} />
