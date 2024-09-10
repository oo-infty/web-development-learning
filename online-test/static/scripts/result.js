const demonstration = document.querySelector(".result-demonstration");

async function fetchQueryResult() {
  const params = new URLSearchParams(window.location.search);
  const value = params.get("query")
  const request = {
    login_id: 1,
    kind: (value ? value : "best"),
  };

  try {
    const resp = await fetch("../api/query", {
      method: "POST",
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
      credentials: "same-origin",
    });

    const text = await resp.text();

    if (!resp.ok){

      if (text.search("Could not serve without logging in") != -1) {
        window.alert("Error: Not logined. You must login first to query result!");
        location.assign("../login.html");
      } else {
        window.alert(`Internal Server Error: ${text}`);
        location.assign("../login.html");
      }
      return null;
    }

    return JSON.parse(text);
  } catch (err) {
    window.alert("Error: Could not query result");
    return null;
  }
}

async function generateDemonstration() {
  const res = await fetchQueryResult();
  const score = res.result[0].score;
  const min = Math.floor(res.result[0].duration / 60);
  const sec = res.result[0].duration % 60;

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
