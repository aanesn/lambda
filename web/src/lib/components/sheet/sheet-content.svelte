<script lang="ts">
	import { Dialog } from "bits-ui"
	import type { Snippet } from "svelte"
	import { cx } from "$lib/utils"
	import SheetOverlay from "./sheet-overlay.svelte"

	let {
		class: className,
		children,
		...restProps
	}: Dialog.ContentProps & {
		children: Snippet
	} = $props()
</script>

<Dialog.Portal>
	<SheetOverlay />
	<Dialog.Content
		class={cx(
			"fixed inset-y-0 start-0 z-50 flex h-full w-3/4 flex-col gap-4 border-e bg-black shadow-lg transition ease-in-out data-[state=closed]:animate-out data-[state=closed]:duration-300 data-[state=closed]:slide-out-to-start data-[state=open]:animate-in data-[state=open]:duration-500 data-[state=open]:slide-in-from-start sm:max-w-sm",
			className
		)}
		{...restProps}
	>
		{@render children?.()}
	</Dialog.Content>
</Dialog.Portal>
