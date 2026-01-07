<script lang="ts" module>
	export const sheetVariants = cva({
		base: "fixed z-50 flex flex-col gap-4 bg-neutral-950 shadow-lg transition ease-in-out data-[state=closed]:animate-out data-[state=closed]:duration-300 data-[state=open]:animate-in data-[state=open]:duration-500",
		variants: {
			side: {
				top: "inset-x-0 top-0 h-auto border-b data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top",
				bottom: "inset-x-0 bottom-0 h-auto border-t data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom",
				left: "inset-y-0 start-0 h-full w-3/4 border-e data-[state=closed]:slide-out-to-start data-[state=open]:slide-in-from-start sm:max-w-sm",
				right: "inset-y-0 end-0 h-full w-3/4 border-s data-[state=closed]:slide-out-to-end data-[state=open]:slide-in-from-end sm:max-w-sm"
			}
		}
	})
	export type Side = VariantProps<typeof sheetVariants>["side"]
</script>

<script lang="ts">
	import { Dialog } from "bits-ui"
	import type { VariantProps } from "cva"
	import type { ComponentProps, Snippet } from "svelte"
	import { cva, cx } from "$lib/utils"
	import SheetOverlay from "./sheet-overlay.svelte"

	let {
		class: className,
		side = "left",
		portalProps,
		children,
		...restProps
	}: Dialog.ContentProps & {
		portalProps?: ComponentProps<typeof Dialog.Portal>
		side?: Side
		children: Snippet
	} = $props()
</script>

<Dialog.Portal {...portalProps}>
	<SheetOverlay />
	<Dialog.Content class={cx(sheetVariants({ side }), className)} {...restProps}>
		{@render children?.()}
	</Dialog.Content>
</Dialog.Portal>
