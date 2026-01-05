<script lang="ts" module>
	import { cva, cx } from "$lib/utils"
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements"

	export const buttonVariants = cva({
		base: "inline-flex shrink-0 items-center justify-center rounded-full whitespace-nowrap transition-all outline-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			intent: {
				primary: "bg-white font-medium text-black shadow-xs hover:bg-white/80",
				secondary: "bg-white/15 shadow-xs hover:bg-white/20",
				outline: "border bg-black shadow-xs hover:bg-white/10"
			},
			size: {
				md: "h-10 gap-x-2 px-4.5 py-2 text-sm",
				lg: "h-12 gap-x-2.5 px-5 py-2"
			}
		}
	})
	export type ButtonIntent = VariantProps<typeof buttonVariants>["intent"]
	export type ButtonSize = VariantProps<typeof buttonVariants>["size"]
	export type ButtonProps = HTMLButtonAttributes &
		HTMLAnchorAttributes & {
			intent?: ButtonIntent
			size?: ButtonSize
		}
</script>

<script lang="ts">
	let {
		class: className,
		intent = "primary",
		size = "md",
		href = undefined,
		children,
		...restProps
	}: ButtonProps = $props()
</script>

{#if href}
	<a class={cx(buttonVariants({ intent, size }), className)} {href} {...restProps}>
		{@render children?.()}
	</a>
{:else}
	<button class={cx(buttonVariants({ intent, size }), className)} {...restProps}>
		{@render children?.()}
	</button>
{/if}
