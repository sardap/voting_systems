<script setup lang="ts">
import { onMounted, ref, type PropType } from 'vue'
import { useSortable } from '@vueuse/integrations/useSortable'
import { move_element } from '@/utils'

interface VoteOption {
  name: string
  index: number
}

const props = defineProps({
  modelValue: {
    type: Object as PropType<VoteOption[]>,
    required: true
  },
  reverse_order: {
    type: Boolean,
    required: false,
    default: false
  },
  rank_title: {
    type: String,
    required: false,
    default: 'Rank'
  }
})

const emits = defineEmits<{
  (e: 'update:modelValue', value: VoteOption[]): void
}>()

const options = ref<VoteOption[]>(props.modelValue)
const change_inputs = ref<number[]>([])

const el = ref<HTMLElement | null>(null)

function refresh_change_inputs() {
  console.log(`REFRESH CHANGE INPUTS ${options.value.length}`)
  if (props.reverse_order) {
    change_inputs.value = options.value.map((_, index) => options.value.length - index)
  } else {
    change_inputs.value = options.value.map((_, index) => index + 1)
  }
  console.log(`UPSTREAMING ${JSON.stringify(options.value)}`)
  emits('update:modelValue', options.value)
}

onMounted(() => {
  refresh_change_inputs()
})

useSortable(el, props.modelValue, {
  animation: 150,
  draggable: '.item',
  onUpdate: (e: { oldIndex: number; newIndex: number }) => {
    options.value = move_element(options.value, e.oldIndex - 1, e.newIndex - 1)
    refresh_change_inputs()
  }
})

async function input_change(old_index: number, new_index: number) {
  if (new_index > options.value.length - 1 || new_index < 0) {
    return
  }

  // Move everything down
  options.value = move_element(options.value, old_index, new_index)
  refresh_change_inputs()
}
</script>

<template>
  <div>
    <div ref="el" class="table">
      <div class="row header">
        <div class="col rank">
          <p>{{ rank_title }}</p>
        </div>
        <div class="col">
          <p>Name</p>
        </div>
      </div>
      <div v-for="(option, i) in options" :key="option.index" class="row item">
        <div class="col rank">
          <input
            class="input-number"
            type="number"
            @change="() => input_change(i, change_inputs[i] - 1)"
            v-model="change_inputs[i]"
          />
        </div>
        <div class="col">
          <p>{{ option.name }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
div.row:hover {
  background-color: #e8ab8c;
}

div.row:nth-child(2n):hover {
  background-color: #e8ab8c;
}

div.row:nth-child(2n) {
  background-color: #fad8c0;
}

div.row {
  background-color: #fde5d0;
}

div.row.header {
  background-color: #a2f4b9;
}

div.table {
  display: table;
  max-width: 800px;
  width: 90%;
  border-collapse: separate;
  border-spacing: 0;
}

div.row {
  display: table-row;
}

div.col {
  display: table-cell;
  border: 1px solid black;
  padding: 8px;
  text-align: left;
  vertical-align: top;
}

div.col.rank {
  width: 10px;
}

.submit {
  margin-top: 20px;
  max-width: 800px;
  width: 70%;
  height: 80px;
}

.input-number {
  width: 3em;
  text-align: center;
}

td {
  text-align: left;
}

/* Chrome, Safari, Edge, Opera */
input[type='number']::-webkit-inner-spin-button,
input[type='number']::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type='number'] {
  -moz-appearance: textfield;
}
</style>
