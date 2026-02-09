<script lang="ts">
	import { Avatar } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: Avatar.FallbackProps = $props()
</script>

<Avatar.Fallback
	class={cx("flex size-full items-center justify-center rounded-full bg-neutral-900", className)}
	{...restProps}
/>
