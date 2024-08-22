<script setup lang="ts">
import { ref, type PropType } from 'vue'
import type { VoteOption } from '@/utils'
import RankedChoiceOptional from '@/components/voting/RankedChoiceOptional.vue'

const props = defineProps({
  options: {
    type: Array as PropType<VoteOption[]>,
    required: true
  }
})

const emit = defineEmits<{
  (e: 'updated', value: number[]): void
}>()

interface QuotaPreferentialVicLabor2024BallotOption extends VoteOption {}

const options = ref<QuotaPreferentialVicLabor2024BallotOption[]>(
  props.options.map((option) => {
    return {
      ...option
    }
  })
)
</script>

<template>
  <div>
    <RankedChoiceOptional
      :options="options"
      rank_title="Preference"
      @updated="(updated) => emit('updated', updated)"
    />
  </div>
</template>

<style scoped></style>
