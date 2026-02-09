<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import * as Tooltip from "$lib/components/tooltip"
	import { cx } from "$lib/utils"
	import { SIDEBAR_WIDTH, SIDEBAR_WIDTH_ICON } from "./constants.js"
	import { setSidebar } from "./context.svelte.js"

	let {
		open = $bindable(false),
		onOpenChange = () => {},
		class: className,
		style,
		children,
		...restProps
	}: HTMLAttributes<HTMLDivElement> & {
		open?: boolean
		onOpenChange?: (open: boolean) => void
	} = $props()

	setSidebar({
		open: () => open,
		setOpen: (value: boolean) => {
			open = value
			onOpenChange(value)
		}
	})
</script>

<Tooltip.Provider delayDuration={0}>
	<div
		style="--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}; {style}"
		class={cx("group/sidebar-wrapper flex min-h-svh w-full", className)}
		{...restProps}
	>
		{@render children?.()}
	</div>
</Tooltip.Provider>
