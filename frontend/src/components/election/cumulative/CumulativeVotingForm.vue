<script setup lang="ts">
import { ref, type PropType } from 'vue'
import { ElectionType, submit_generic_vote, type CumulativeElection } from '@/backend'
import type { VoteOption } from '@/utils'

const props = defineProps({
  election: {
    type: Object as PropType<CumulativeElection>,
    required: true
  },
  options: {
    type: Array as PropType<VoteOption[]>,
    required: true
  },
  vote_token: {
    type: String,
    required: false
  }
})

const emits = defineEmits({
  complete: () => true,
  error: (error: string) => true
})

const loading = ref<boolean>(false)

const votes = ref<number[]>(props.options.map(() => 0))
const refresh_votes = ref<number>(0)

async function submit() {
  if (vote_sum() <= 0) {
    emits(`error`, 'you have voted too much')
    return
  }

  loading.value = true

  const response = await submit_generic_vote(
    ElectionType.Cumulative,
    props.election.id,
    props.vote_token,
    votes.value
  )
  const data = await response.text()
  loading.value = false
  if (!response.ok) {
    emits(`error`, data)
    return
  }

  emits(`complete`)
}

function vote_changed() {
  refresh_votes.value++
}

function vote_sum(): number {
  let sum = 0
  for (const number of votes.value) {
    sum += number
  }
  return sum
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <p :key="refresh_votes">Votes Remaining {{ election.max_votes - vote_sum() }}</p>
    <div>
      <div ref="el" class="table">
        <div class="row header">
          <div class="col">
            <p>Name</p>
          </div>
          <div class="col">
            <p>Votes</p>
          </div>
        </div>
        <div v-for="option in options" :key="option.index" class="row">
          <div class="col">
            <p>{{ option.name }}</p>
          </div>
          <div class="col">
            <input type="number" v-model="votes[option.index]" @change="vote_changed" />
          </div>
        </div>
      </div>
    </div>
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped>
.selected {
  background-color: red;
}

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
  width: 70%;
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
  text-align: center;
}

.submit {
  margin-top: 20px;
  max-width: 800px;
  width: 70%;
  height: 80px;
}
</style>
