<script lang="ts">
	import { Dialog } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: Dialog.OverlayProps = $props()
</script>

<Dialog.Overlay
	class={cx(
		"fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0",
		className
	)}
	{...restProps}
/>
