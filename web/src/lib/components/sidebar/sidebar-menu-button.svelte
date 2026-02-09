<script lang="ts">
	import { mergeProps } from "bits-ui"
	import type { VariantProps } from "cva"
	import type { ComponentProps, Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"
	import * as Tooltip from "$lib/components/tooltip"
	import { cva, cx } from "$lib/utils"
	import { useSidebar } from "./context.svelte.js"

	const sidebarMenuButtonVariants = cva({
		base: "flex w-full items-center gap-2.5 overflow-hidden rounded-lg p-2 text-start text-sm outline-hidden transition-[width,height,padding] group-data-[state=collapsed]:size-8! group-data-[state=collapsed]:p-1.5! [&>span:last-child]:truncate [&>svg]:shrink-0",
		variants: {
			intent: {
				primary:
					"hover:bg-neutral-900 text-neutral-500 hover:text-white active:bg-neutral-900 active:text-white data-[active=true]:bg-neutral-900 data-[active=true]:text-white",
				secondary: "",
				outline:
					"bg-neutral-950 hover:bg-neutral-900 shadow-[0_0_0_1px_rgba(255,255,255,0.1)]"
			},
			size: {
				md: "h-8 text-sm",
				lg: "h-12 text-sm"
			}
		}
	})

	let {
		class: className,
		children,
		child,
		intent = "primary",
		size = "md",
		isActive = false,
		tooltipContent,
		tooltipContentProps,
		...restProps
	}: HTMLAttributes<HTMLButtonElement> & {
		isActive?: boolean
		intent?: VariantProps<typeof sidebarMenuButtonVariants>["intent"]
		size?: VariantProps<typeof sidebarMenuButtonVariants>["size"]
		tooltipContent?: Snippet | string
		tooltipContentProps?: ComponentProps<typeof Tooltip.Content>
		child?: Snippet<[{ props: Record<string, unknown> }]>
	} = $props()

	const sidebar = useSidebar()

	const buttonProps = $derived({
		class: cx(sidebarMenuButtonVariants({ intent, size }), className),
		"data-size": size,
		"data-active": isActive,
		...restProps
	})
</script>

{#snippet Button({ props }: { props?: Record<string, unknown> })}
	{@const mergedProps = mergeProps(buttonProps, props)}
	{#if child}
		{@render child({ props: mergedProps })}
	{:else}
		<button {...mergedProps}>
			{@render children?.()}
		</button>
	{/if}
{/snippet}

{#if !tooltipContent}
	{@render Button({})}
{:else}
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				{@render Button({ props })}
			{/snippet}
		</Tooltip.Trigger>
		<Tooltip.Content
			side="right"
			align="center"
			hidden={sidebar.state !== "collapsed" || sidebar.isMobile}
			{...tooltipContentProps}
		>
			{#if typeof tooltipContent === "string"}
				{tooltipContent}
			{:else if tooltipContent}
				{@render tooltipContent()}
			{/if}
		</Tooltip.Content>
	</Tooltip.Root>
{/if}
