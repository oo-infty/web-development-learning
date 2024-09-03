const demonstration = document.querySelector(".result-demonstration");

function generateDemonstration() {
  const scoreNode = document.querySelector("#result-score");
  const score = Number(scoreNode.textContent);
  demonstration.removeChild(scoreNode);

  const minutesNode = document.querySelector("#result-minutes");
  const min = Number(minutesNode.textContent);
  demonstration.removeChild(minutesNode);

  const secondsNode = document.querySelector("#result-seconds");
  const sec = Number(secondsNode.textContent);
  demonstration.removeChild(secondsNode);

  const paraTime = demonstration.querySelector("#card-time p:last-child");
  paraTime.textContent = (sec !== 0 ? `${min} min ${sec} s` : `${min} min`);

  const paraScore = demonstration.querySelector("#card-score p:last-child");
  paraScore.textContent = `${score} pts`;

  const paraLevelHeader = demonstration.querySelector("#card-level p:first-child");
  const paraLevelValue = demonstration.querySelector("#card-level p:last-child");

  if (score >= 90) {
    paraLevelHeader.textContent = "Congradulations! You are now";
    paraLevelValue.textContent = "ADVANCED CERTIFIED SYSTEM ADMINISTRATOR";
  } else if (score >= 75) {
    paraLevelHeader.textContent = "Congradulations! You are now";
    paraLevelValue.textContent = "CERTIFIED SYSTEM ADMINISTRATOR";
  } else {
    paraLevelHeader.textContent = "Never mind. You have already made";
    paraLevelValue.textContent = "A GREAT PROGRESS";
  }
}

function main() {
  generateDemonstration();
}

main();
