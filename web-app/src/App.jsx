/* eslint-disable react/prop-types */
import { useState } from "react";
import QRCode from 'react-qr-code';
import "./App.css";
import "./profilestyle.css";

function App() {
  const [profileInfo, setProfileInfo] = useState(null);
  const [linkedInPath, setLinkedInPath] = useState("");
  const [authToken, setAuthToken] = useState("");
  const [userEmail, setUserEmail] = useState("");
  const [userPassword, setUserPassword] = useState("");
  const [jobRole, setJobRole] = useState("");

  async function postAuth(data = {username : userEmail, password : userPassword}) {
    const response = await fetch(`http://127.0.0.1:8091/`, {
      method: "POST",
      mode: "cors", // no-cors, *cors, same-origin
      cache: "default", // *default, no-cache, reload, force-cache, only-if-cached
      credentials: "same-origin", // include, *same-origin, omit
      headers: {
        "Content-Type": "application/json",
      },
      redirect: "follow", // manual, *follow, error
      referrerPolicy: "no-referrer-when-downgrade", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
      body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
    
    response.json().then(
      (resp) => {
        setAuthToken("Bearer " + resp.token);
      },
      (resp) => {
        setAuthToken("Failed to get valid response.");
      },
    );
  }

  async function getData(profile_user = "") {

    const myHeaders = new Headers();
    myHeaders.append('Content-Type', 'application/json');
    myHeaders.append('Authorization', authToken);
    
    const response = await fetch(`http://127.0.0.1:8091/${profile_user}`, {
      method: 'GET',
      headers: myHeaders,
    })
    
    response.json().then(
      (resp) => {
        setProfileInfo(resp);
      },
      (resp) => {
        setProfileInfo("Failed to get valid response.");
      },
    );
  }
  
  function ProfileCard(){

    if(!profileInfo){
      return (<></>)
    }

    return (
      <div className="profile-card">
        <QRCodeImg url_value={`https://www.linkedin.com/in/${linkedInPath}`} qr_img_size={130} />
        <img
          className="background-img"
          src={profileInfo.cover_img}
          alt="Background"
        />
        <div className="profile-info">
          <img
            className="profile-img"
            src={profileInfo.profile_img}
            alt="Profile"
          />
        </div>
        <div className="profile-desc-container">
          <div className="profile-desc">
            <p className="profile-name">{profileInfo.profile_name}</p>
            <p id="job-role">{jobRole}</p>
            <p className="profile-about">{profileInfo.profile_about}</p>
          </div>
          <RenderCompanies companies={profileInfo.company_img_title} />
          </div>
        </div>
    );
  }

  return (
    <>
      <h1>Card Builder</h1>
      <div className="card">
        <InputEmail setUserEmail={setUserEmail} />
        <InputPassword setUserPassword={setUserPassword} />
        <button
          onClick={() =>
            postAuth()
          }
        >
          Authenticate
        </button>
        <InputAuthToken setAuthToken={setAuthToken} />
        <p>{authToken}</p>
        <InputData setLinkedInPath={setLinkedInPath} />
        <button
          onClick={() =>
            getData(linkedInPath)
          }
        >
          Build Card
        </button>
      </div>
      <div className="card">
        <ProfileCard />
        <InputJobRole setJobRole={setJobRole} />
      </div>
      <p className="more-info">No support available.</p>
    </>
  );
}


function InputData({ setLinkedInPath }) {
  return (
    <div className="url-path">
      <label>Please Insert LinkedIn User Profile: </label>
      <input
        type="text"
        className="user-input"
        onChange={(elem) => setLinkedInPath(elem.target.value)}
        required
      />
    </div>
  );
}

function InputJobRole({ setJobRole }) {
  return (
    <div className="job-path">
      <label>Please Insert Your Job Role: </label>
      <input
        type="text"
        className="user-input"
        onChange={(elem) => setJobRole(elem.target.value)}
        required
      />
    </div>
  );
}

function InputEmail({ setUserEmail }) {
  return (
    <div className="email-path">
      <label>Please Insert you LinkedIn email: </label>
      <input
        type="text"
        className="user-input"
        onChange={(elem) => setUserEmail(elem.target.value)}
        required
      />
    </div>
  );
}

function InputPassword({ setUserPassword }) {
  return (
    <div className="password-path">
      <label>Please insert your LinkedIn password: </label>
      <input
        type="password"
        className="user-input"
        onChange={(elem) => setUserPassword(elem.target.value)}
        required
      />
    </div>
  );
}

function InputAuthToken({ setAuthToken }) {
  return (
    <div className="token-path">
      <label>Auth Token: </label>
      <input
        type="text"
        className="user-input"
        onChange={(elem) => setAuthToken("Bearer " + elem.target.value)}
        required
      />
    </div>
  );
}

function RenderCompanies({companies}) {
  return (
    <div className="companies">
      {companies.map((company) => {
        return (
          <div key={company.name}>
            <img
              alt="Company Logo"
              className="company-img"
              src={company.img_url}
            />
            <div className="company-name">
              <p>{company.name}</p>
            </div>
          </div>
        );
      })}
    </div>
  );
}

function QRCodeImg({ url_value, qr_img_size }) {
  return (
    <div id="qr-img">
      <QRCode
        title="QR LinkedIn Profile"
        value={url_value}
        bgColor="#FFFFFF"
        fgColor="#000000"
        size={qr_img_size}
      />
    </div>
  );
}

export default App;
