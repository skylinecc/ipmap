var ipmap = L.map('mapid').setView([20, 0], 2);

L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
}).addTo(ipmap);

var alreadyAdded = new Set();

var ipIndex = 0;

function addMarkers(jsonText) {
	console.log(jsonText)
	for(var i = 1; i < jsonText.length; i++) {
		var obj = jsonText[i];

		if (alreadyAdded.has(obj.ip)) {
			continue;
		}

		console.log(obj.ip);

		ipIndex = ipIndex + 1;

		document.getElementById("totalips").innerHTML = ipIndex + " Unique IPs";

		L.marker([obj.latitude, obj.longitude]).addTo(ipmap)
// .bindpopup(obj.ip);
		alreadyAdded.add(obj.ip);
	}
}

var jsonText;

(function loop() {
	setTimeout(function () {
		fetch("map.json")
  	.then(res => res.json())
  	.then(json => jsonText = json)
  	.then(() => addMarkers(jsonText));
		loop()
	}, 1000);
}());

//var popup = L.popup();
