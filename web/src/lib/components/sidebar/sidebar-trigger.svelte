<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import type { ComponentProps } from "svelte"
	import { useSidebar } from "./context.svelte"
	import Sidebar from "$lib/assets/sidebar.svg?raw"

	let {
		class: className,
		onclick,
		...restProps
	}: ComponentProps<typeof Button> & {
		onclick?: (e: MouseEvent) => void
	} = $props()

	const sidebar = useSidebar()
</script>

<Button
	intent="ghost"
	size="icon"
	onclick={(e) => {
		onclick?.(e)
		sidebar.toggle()
	}}
	{...restProps}
>
	{@html Sidebar}
</Button>
