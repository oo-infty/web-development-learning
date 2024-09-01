const demonstration = document.querySelector(".result-demonstration");

function generateDemonstration() {
  const scoreNode = document.querySelector("#result-score");
  const score = Number(scoreNode.textContent);
  demonstration.removeChild(scoreNode);

  function createEmphasis(text, capitalize) {
    const span = document.createElement("span");
    span.setAttribute("class", "result-emphasis");
    span.textContent = text;
    return span;
  }

  const paraScore = document.createElement("p");
  paraScore.appendChild(document.createTextNode("Your score is"));
  paraScore.appendChild(document.createElement("br"));
  paraScore.appendChild(createEmphasis(`${score} pts`));
  demonstration.appendChild(paraScore);

  if (score >= 75) {
    const paraLevel = document.createElement("p");
    paraLevel.appendChild(document.createTextNode("Congradulations! You are now"));
    paraLevel.appendChild(document.createElement("br"));

    if (score >= 90) {
      paraLevel.appendChild(createEmphasis("ADVANCED CERTIFIED SYSTEM ADMINISTRATOR"));
      demonstration.appendChild(paraLevel);
    } else {
      paraLevel.appendChild(createEmphasis("CERTIFIED SYSTEM ADMINISTRATOR"));
      demonstration.appendChild(paraLevel);
    }
  } else {
    const paraLevel = document.createElement("p");
    paraLevel.appendChild(document.createTextNode("Never mind. You have already made"));
    paraLevel.appendChild(document.createElement("br"));
    paraLevel.appendChild(createEmphasis("A GREAT PROGRESS"));
    demonstration.appendChild(paraLevel);
  }
}

function main() {
  generateDemonstration();
}

main();
