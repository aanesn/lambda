<script lang="ts">
	import { cx } from "$lib/utils"
	import { DropdownMenu } from "bits-ui"

	let { class: className, sideOffset = 8, ...others }: DropdownMenu.ContentProps = $props()
</script>

<DropdownMenu.Portal>
	<DropdownMenu.Content
		{sideOffset}
		{...others}
		class={cx(
			"data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 bg-neutral-900 min-w-52 border rounded-2xl p-1 shadow-md duration-100 data-[side=inline-start]:slide-in-from-right-2 data-[side=inline-end]:slide-in-from-left-2 z-50 overflow-x-hidden overflow-y-auto outline-none data-[state=closed]:overflow-hidden",
			className
		)}
	/>
</DropdownMenu.Portal>
