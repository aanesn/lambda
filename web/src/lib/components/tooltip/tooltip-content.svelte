<script lang="ts">
	import { Tooltip } from "bits-ui"
	import type { ComponentProps } from "svelte"
	import { cx } from "$lib/utils"

	let {
		class: className,
		sideOffset = 0,
		side = "top",
		children,
		arrowClasses,
		portalProps,
		...restProps
	}: Tooltip.ContentProps & {
		arrowClasses?: string
		portalProps?: ComponentProps<typeof Tooltip.Portal>
	} = $props()
</script>

<Tooltip.Portal {...portalProps}>
	<Tooltip.Content
		{sideOffset}
		{side}
		class={cx(
			"z-50 w-fit origin-(--bits-tooltip-content-transform-origin) animate-in rounded-md bg-white px-3 py-1.5 text-xs text-balance text-black fade-in-0 zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-end-2 data-[side=right]:slide-in-from-start-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
			className
		)}
		{...restProps}
	>
		{@render children?.()}
		<Tooltip.Arrow>
			{#snippet child({ props })}
				<div
					class={cx(
						"z-50 size-2.5 rotate-45 rounded-xs bg-white",
						"data-[side=top]:translate-x-1/2 data-[side=top]:translate-y-[calc(-50%+2px)]",
						"data-[side=bottom]:-translate-x-1/2 data-[side=bottom]:-translate-y-[calc(-50%+1px)]",
						"data-[side=right]:translate-x-[calc(50%+2px)] data-[side=right]:translate-y-1/2",
						"data-[side=left]:-translate-y-[calc(50%-3px)]",
						arrowClasses
					)}
					{...props}
				></div>
			{/snippet}
		</Tooltip.Arrow>
	</Tooltip.Content>
</Tooltip.Portal>
