<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import type { ComponentProps } from "svelte"
	import { cx } from "$lib/utils"

	let {
		sideOffset = 6,
		portalProps,
		class: className,
		...restProps
	}: DropdownMenu.ContentProps & {
		portalProps?: ComponentProps<typeof DropdownMenu.Portal>
	} = $props()
</script>

<DropdownMenu.Portal {...portalProps}>
	<DropdownMenu.Content
		{sideOffset}
		class={cx(
			"z-50 max-h-(--bits-dropdown-menu-content-available-height) min-w-56 origin-(--bits-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto rounded-xl border bg-black p-1 shadow-md outline-none data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-end-2 data-[side=right]:slide-in-from-start-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
			className
		)}
		{...restProps}
	/>
</DropdownMenu.Portal>
