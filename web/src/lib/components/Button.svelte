<script lang="ts" module>
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements"
	import { cva, cx } from "$lib/utils"

	export const buttonVariants = cva({
		base: "inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			intent: {
				primary: "bg-white font-medium text-black shadow-xs hover:bg-white/80",
				secondary: "bg-white/15 shadow-xs hover:bg-white/20",
				outline: "border bg-black shadow-xs hover:bg-white/10",
				ghost: "hover:bg-white/10"
			},
			size: {
				md: "h-10 gap-x-2 rounded-full px-4.5 py-2 text-sm",
				lg: "h-12 gap-x-2.5 rounded-full px-5 py-2",
				icon: "size-8 rounded-lg"
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
