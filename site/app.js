// Define the API endpoint
const apiEndpoint = "/api?visualizer"; // Replace with your actual API URL

// Function to fetch data from the API and process it
async function fetchData() {
  try {
    // Step 1: Fetch data from the API
    const response = await fetch(apiEndpoint);

    // Check if the response is successful (status code 200-299)
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }

    // Step 2: Parse the response as JSON
    const data = await response.json();

    // Step 3: Process the data for use in the chart
    processDataAndRenderChart(data);
  } catch (error) {
    // Handle errors (network issues, parsing issues, etc.)
    console.error("There was a problem fetching the data:", error);
  }
}

// Function to process the fetched data and render the chart
function processDataAndRenderChart(data) {
  // Extract labels (time) and datasets (value) for the chart
  //const labels0 = data[0].data.map((item) =>
  //  new Date(item.time * 1000).toLocaleTimeString(),
  //); // Convert UNIX timestamp to time
  //const labels1 = data[1].data.map((item) =>
  //  new Date(item.time * 1000).toLocaleTimeString(),
  //);
  //const labels2 = data[1].data.map((item) =>
  //  new Date(item.time * 1000).toLocaleTimeString(),
  //);
  //const labels3 = data[1].data.map((item) =>
  //  new Date(item.time * 1000).toLocaleTimeString(),
  //);
  //const labels = labels0.concat(labels1, labels2, labels3);

  const labels = [];
  data.forEach((data, index) => {
    labels.push(data.data.map((item) => item.time));
  });
  const datasets = [];

  // Step 4: Generate datasets for each device/measure
  data.forEach((deviceData, index) => {
    const measureData = deviceData.data.map((item) => item.value); // Extract the 'value' for the y-axis

    datasets.push({
      label: `${deviceData.device} - ${deviceData.measure}`, // e.g., "Board1 - humidity"
      data: measureData,
      borderColor: `hsl(${(index * 30) % 360}, 100%, 50%)`, // Unique color for each device
      backgroundColor: `hsl(${(index * 30) % 360}, 100%, 80%)`,
      fill: false,
      hidden: false, // Initially visible
    });
  });

  // Step 5: Render the chart with the processed data
  renderChart(labels, datasets);
}

// Function to render the chart using Chart.js
function renderChart(labels, datasets) {
  const ctx = document.getElementById("myChart").getContext("2d");

  // Create the chart
  new Chart(ctx, {
    type: "line", // Line chart type
    data: {
      labels: labels.flat(), // X-axis labels (time)
      datasets: datasets, // Data for each line (device/measure)
    },
    options: {
      responsive: true,
      scales: {
        y: {
          beginAtZero: true,
        },
        x: {
          type: "time",
          time: {
            unit: "hour",
            displayFormats: {
              hour: "MM-dd-HH",
            },
          },
        },
      },
    },
  });
}

// Call the fetchData function when the page loads
window.onload = fetchData;
