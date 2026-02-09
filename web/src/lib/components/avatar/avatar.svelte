<script lang="ts">
	import { Avatar } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: Avatar.RootProps = $props()
</script>

<Avatar.Root
	class={cx("relative flex size-8 shrink-0 overflow-hidden rounded-full", className)}
	{...restProps}
/>
