const tableBody = document.querySelector(".statistics-table tbody");

async function fetchQueryResult() {
  const loginId = sessionStorage.getItem("loginId");

  if (!loginId) {
    window.alert("Error: Not logined. You must login first to participate in the test!");
    location.assign("../login.html");
    return;
  }

  const request = {
    login_id: Number(loginId),
    kind: "all",
  };

  try {
    const resp = await fetch("../api/query", {
      method: "POST",
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
      credentials: "same-origin",
    });

    const text = await resp.text();

    if (!resp.ok) {

      if (text.search("Could not serve without logging in") != -1) {
        window.alert("Error: Not logined. You must login first to query result!");
        location.assign("../login.html");
      } else {
        window.alert(`Internal Server Error: ${text}`);
        location.assign("../index.html");
      }
      return null;
    }

    return JSON.parse(text);
  } catch (err) {
    window.alert("Error: Could not query result");
    return null;
  }
}

function createTableRow(record, num) {
  const row = document.createElement("tr");

  const number = document.createElement("td");
  number.textContent = num;
  row.appendChild(number);

  const score = document.createElement("td");
  score.textContent = `${Math.round(record.score * 10) / 10} pts`;
  row.appendChild(score);

  const endTime = document.createElement("td");
  endTime.textContent = record.end_time;
  row.appendChild(endTime);

  const duration = document.createElement("td");
  const min = Math.floor(record.duration / 60);
  const sec = record.duration % 60;
  duration.textContent = (sec !== 0 ? `${min} min ${sec} s` : `${min} min`);
  row.appendChild(duration);

  return row;
}

async function generateTableBody() {
  const res = await fetchQueryResult();
  let num = 0;

  if (res.result.length !== 0) {
    res.result.forEach(record => {
      ++num;
      const row = createTableRow(record, num);
      console.log(row);
      tableBody.appendChild(row);
    });
  } else {
    const row = document.createElement("tr");

    for (let i = 0; i < 4; ++i) {
      const slash = document.createElement("td");
      slash.textContent = "/";
      row.appendChild(slash);
    }

    tableBody.appendChild(row);
  }
}

function main() {
  generateTableBody().then(() => {});
}

main();
