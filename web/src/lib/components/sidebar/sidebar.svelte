<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import * as Sheet from "$lib/components/sheet"
	import { cx, SIDEBAR_WIDTH } from "$lib/utils"
	import { useSidebar } from "./context.svelte"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLDivElement> = $props()

	const sidebar = useSidebar()
</script>

{#if sidebar.isMobile}
	<Sheet.Root
		bind:open={() => sidebar.openMobile, (v) => sidebar.setOpenMobile(v)}
		{...restProps}
	>
		<Sheet.Content class="w-(--sidebar-width) p-0" style="--sidebar-width: {SIDEBAR_WIDTH};">
			<div class="flex h-full w-full flex-col">
				{@render children?.()}
			</div>
		</Sheet.Content>
	</Sheet.Root>
{:else}
	<div class="group peer hidden md:block" data-state={sidebar.state}>
		<div
			class="relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear group-data-[state=collapsed]:w-(--sidebar-width-icon)"
		></div>
		<div
			class={cx(
				"fixed inset-y-0 start-0 z-10 hidden h-svh w-(--sidebar-width) transition-[left,right,width] duration-200 ease-linear group-data-[state=collapsed]:w-(--sidebar-width-icon) md:flex",
				className
			)}
			{...restProps}
		>
			<div class="flex h-full w-full flex-col bg-neutral-950">
				{@render children?.()}
			</div>
		</div>
	</div>
{/if}
