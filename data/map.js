var ipmap = L.map('mapid').setView([20, 0], 2);

	L.tileLayer('https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token=pk.eyJ1IjoibWFwYm94IiwiYSI6ImNpejY4NXVycTA2emYycXBndHRqcmZ3N3gifQ.rJcFIG214AriISLbB6B5aw', {
		maxZoom: 18,
		attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, ' +
			'Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
		id: 'mapbox/streets-v11',
		tileSize: 512,
		zoomOffset: -1
	}).addTo(ipmap);

var alreadyAdded = new Set();

function addMarkers(jsonText) {
	console.log(jsonText)
	for(var i = 0; i < jsonText.length; i++) {
		var obj = jsonText[i];

		if (alreadyAdded.has(obj.ip)) {
			continue;
		}

		console.log(obj.ip);

		document.getElementById("totalIps").innerHTML = jsonText.length + " Unique IP's";

		L.marker([obj.latitude, obj.longitude]).addTo(ipmap).bindPopup("<b>" + obj.ip + "</b> - " + obj.city)
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
