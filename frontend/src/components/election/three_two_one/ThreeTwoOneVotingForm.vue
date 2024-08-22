<script setup lang="ts">
import { ref, type PropType } from 'vue'
import {
  ElectionType,
  submit_generic_vote,
  type GenericElection,
  GoodOkBad,
  from_good_ok_bad_to_int
} from '@/backend'
import type { VoteOption } from '@/utils'

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
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
interface ThreeTwoOneOption extends VoteOption {
  rating: GoodOkBad
}

const loading = ref<boolean>(false)
const options = ref<ThreeTwoOneOption[]>(
  props.options.map((option) => {
    return {
      ...option,
      rating: GoodOkBad.Bad
    }
  })
)

async function submit() {
  const candidates = props.election.options

  const vote: number[] = []
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const points = from_good_ok_bad_to_int(
      options.value.find((option) => option.index == option_index)?.rating as GoodOkBad
    )
    vote.push(points)
  }

  loading.value = true

  const response = await submit_generic_vote(
    ElectionType.ThreeTwoOne,
    props.election.id,
    props.vote_token,
    vote
  )
  const data = await response.text()
  loading.value = false
  if (!response.ok) {
    emits(`error`, data)
    console.log(`ERROR: ${data}`)
    return
  }

  emits(`complete`)
}

function change_rating(i: number, rating: GoodOkBad) {
  options.value[i].rating = rating
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <div>
      <div ref="el" class="table">
        <div class="row header">
          <div class="col">
            <p>Name</p>
          </div>
          <div class="col">
            <p>Bad, Ok, or Good</p>
          </div>
        </div>
        <div v-for="(option, i) in options" :key="option.index" class="row">
          <div class="col">
            <p>{{ option.name }}</p>
          </div>
          <div class="col">
            <button
              :class="`circle-button ` + (option.rating == GoodOkBad.Bad ? `bad` : ``)"
              @click="change_rating(i, GoodOkBad.Bad)"
            >
              Bad
            </button>
            <button
              :class="`circle-button ` + (option.rating == GoodOkBad.Ok ? `ok` : ``)"
              @click="change_rating(i, GoodOkBad.Ok)"
            >
              OK
            </button>
            <button
              :class="`circle-button ` + (option.rating == GoodOkBad.Good ? `good` : ``)"
              @click="change_rating(i, GoodOkBad.Good)"
            >
              Good
            </button>
          </div>
        </div>
      </div>
    </div>
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped>
@import '../../../assets/voting_table.css';

/* General styles for the circle buttons */
.circle-button {
  display: inline-block;
  width: 60px;
  height: 60px;
  line-height: 60px;
  text-align: center;
  border-radius: 50%;
  font-weight: bold;
  background-color: white;
  cursor: pointer;
  user-select: none;
  margin: 0 5px;
  margin-top: 3px;
  margin-bottom: 3px;
}

.circle-button:hover {
  opacity: 0.8;
}

.circle-button.good {
  background-color: #4caf50;
  color: #ffffff;
}

.circle-button.ok {
  background-color: #ff9800;
  color: #ffffff;
}

.circle-button.bad {
  background-color: #f44336;
  color: #ffffff;
}
</style>
